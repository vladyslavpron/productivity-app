use chrono::prelude::*;
use once_cell::sync::OnceCell;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::entity::*;

use std::thread::JoinHandle;

mod windows_service;

use windows::{
    w,
    Win32::{
        Foundation::{GetLastError, HANDLE, HWND, LPARAM, LRESULT, WPARAM},
        UI::{
            Accessibility::{SetWinEventHook, UnhookWinEvent, HWINEVENTHOOK},
            WindowsAndMessaging::{
                DefWindowProcW, DestroyWindow, DispatchMessageW, GetMessageW, PostQuitMessage,
                RegisterClassW, CS_GLOBALCLASS, EVENT_SYSTEM_FOREGROUND, MSG, WM_CLOSE, WM_DESTROY,
                WNDCLASSW,
            },
        },
    },
};

use self::windows_service::WindowsService;

thread_local! {
    static TX: OnceCell<UnboundedSender<Option<RawWindowEvent>>>= OnceCell::new()
}

pub struct Service {
    tx: UnboundedSender<Option<RawWindowEvent>>,
    cx: UnboundedReceiver<Option<RawWindowEvent>>,
    thread_handle: Option<JoinHandle<()>>,
    db: DatabaseConnection,
    session: session::Model,
}

impl Service {
    pub async fn new(db: DatabaseConnection) -> Self {
        let (tx, cx) = Self::setup_channel();
        let thread_handle = Self::setup_event_messaging_thread(tx.clone());

        // TODO: Application might be restarted at some point, it is better to consiter it the same session when startup time is close to existing session
        let startup_datetime = WindowsService::get_startup_timestamp();

        let record = session::ActiveModel {
            datetime: Set(startup_datetime),
            ..Default::default()
        };

        let session: session::Model = record.insert(&db).await.unwrap();

        Self {
            tx,
            cx,
            thread_handle: Some(thread_handle),
            db,
            session,
        }
    }

    // TODO: make iteration in loop with cx.recv() OR timeout, to track time in current window not only when its changed
    pub async fn spin_loop(&mut self) {
        while let Some(event) = self.cx.recv().await {
            if event.is_none() {
                break;
            };

            // TODO: store processed_event in HashMap for optimisation purposes
            let processed_event = Self::process_event(event.unwrap());

            if let Err(err) = processed_event {
                error!("Error on processing event: {}", err);
                continue;
            }

            let processed_event = processed_event.unwrap();

            // TODO: application might not have "Product name" in its metadata, in this case it's better to use executable name
            let record = event::ActiveModel {
                path: Set(processed_event.path),
                title: Set(processed_event.window_title.clone()),
                offset: Set(processed_event.offset),
                timestamp: Set(Utc::now()),
                session_id: Set(self.session.id),
                app_title: Set(processed_event.app_title),
                ..Default::default()
            };

            let insert_record_result = event::Entity::insert(record).exec(&self.db).await;

            if let Err(err) = insert_record_result {
                error!("Error on inserting event into database: {}", err);
                continue;
            }
        }
    }

    #[allow(dead_code)]
    pub fn stop_loop(&mut self) {
        self.tx.send(None).unwrap();
        self.thread_handle.take().unwrap().join().unwrap();
    }

    fn process_event(event: RawWindowEvent) -> Result<ProcessedWindowEvent, String> {
        let window_title = WindowsService::get_window_title(event.event_id)?;
        let pid = WindowsService::get_process_id(event.event_id)?;
        let process_handle = WindowsService::get_process_handle(pid)?;
        let path = WindowsService::get_process_executable_path(process_handle)?;

        let app_title = WindowsService::get_app_title(path.clone())?;

        Ok(ProcessedWindowEvent {
            window_title,
            pid,
            process_handle,
            path,
            offset: event.timestamp,
            app_title,
        })
    }

    fn setup_channel() -> (
        UnboundedSender<Option<RawWindowEvent>>,
        UnboundedReceiver<Option<RawWindowEvent>>,
    ) {
        unbounded_channel::<Option<RawWindowEvent>>()
    }

    fn setup_event_messaging_thread(tx: UnboundedSender<Option<RawWindowEvent>>) -> JoinHandle<()> {
        std::thread::spawn(move || {
            TX.with(|f| f.set(tx.clone())).unwrap();

            let hook = unsafe {
                SetWinEventHook(
                    EVENT_SYSTEM_FOREGROUND,
                    EVENT_SYSTEM_FOREGROUND,
                    None,
                    Some(Self::win_foreground_change_callback),
                    0,
                    0,
                    0,
                )
            };

            if hook.0 == 0 {
                panic!("Could not setup WinEventHook");
            }

            info!("WinEventHook setup successful!");

            // window class creation and registration for unknown reason doesn't work when separated into another function, hence must stay here

            let mut wc = WNDCLASSW::default();
            let class_name = w!("randomclassname123");
            wc.lpszClassName = class_name;
            wc.lpfnWndProc = Some(Self::window_messaging_proc);
            wc.style = CS_GLOBALCLASS;
            let wc_ptr: *const WNDCLASSW = &wc;

            let class_register_result = unsafe { RegisterClassW(wc_ptr) };

            if class_register_result == 0 {
                let err = unsafe { GetLastError() };

                panic!(
                    "Error on creating window class for messaging window. Windows error code: {}",
                    err.0
                );
            };

            let hwnd = WindowsService::create_messaging_window(wc.lpszClassName);

            let mut msg: MSG = MSG::default();
            let msg_ptr: *mut MSG = &mut msg;

            loop {
                let result = unsafe { GetMessageW(msg_ptr, hwnd, 0, 0) };

                // WM_QUIT or error
                if result.0 == -1 || result.0 == 0 {
                    break;
                }

                unsafe { DispatchMessageW(msg_ptr) };
            }

            info!(
                "Got message loop shut down, no messages will be passed further. Preparing to close message thread"
            );

            TX.with(|f| {
                let tx = f.get().unwrap();
                tx.send(None).unwrap()
            });

            unsafe { UnhookWinEvent(hook) };
        })
    }

    pub extern "system" fn win_foreground_change_callback(
        child_id: HWINEVENTHOOK,
        hook_handle: u32,
        event_id: HWND,
        window_handle: i32,
        object_id: i32,
        thread_id: u32,
        timestamp: u32,
    ) -> () {
        let event = RawWindowEvent {
            child_id,
            hook_handle,
            event_id,
            window_handle,
            object_id,
            thread_id,
            timestamp,
        };

        info!("New event received");

        TX.with(|f| {
            let tx: &UnboundedSender<Option<RawWindowEvent>> = f.get().unwrap();

            tx.send(Some(event)).unwrap();
        });
    }

    pub extern "system" fn window_messaging_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match msg {
            WM_CLOSE => {
                unsafe { PostQuitMessage(0) };
                unsafe { DestroyWindow(hwnd) };
                LRESULT(0)
            }
            WM_DESTROY => {
                unsafe { PostQuitMessage(0) };
                LRESULT(0)
            }

            _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RawWindowEvent {
    /// Hook ID
    /// If this value is [`CHILDID_SELF`], the event was triggered by the object; otherwise, this value is the child ID of the element that triggered the event.
    pub child_id: HWINEVENTHOOK,
    /// Handle to the shared event hook function.
    pub hook_handle: u32,
    /// handle to newly appeared window
    pub event_id: HWND,
    /// Handle to the window that generates the event, or `NULL` if no window is associated with the event.
    pub window_handle: i32,
    /// Identifies the object associated with the event.
    pub object_id: i32,
    /// Identifies the thread that generated the event.
    pub thread_id: u32,
    /// Specifies the time since system startup, in milliseconds, that the event was generated.
    pub timestamp: u32,
}

#[allow(dead_code)]
struct ProcessedWindowEvent {
    pub window_title: String,
    pub pid: u32,
    pub process_handle: HANDLE,
    pub path: String,
    pub offset: u32,
    pub app_title: String,
}
