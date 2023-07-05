use core::slice;
use std::ptr::null_mut;

use chrono::prelude::*;

use windows::{
    core::{HSTRING, PCWSTR},
    h,
    Win32::{
        Foundation::{GetLastError, HANDLE, HWND, MAX_PATH},
        Storage::FileSystem::{GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW},
        System::{
            ProcessStatus::GetModuleFileNameExW,
            SystemInformation::GetTickCount64,
            Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION},
        },
        UI::WindowsAndMessaging::{
            CreateWindowExW, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId,
            HWND_MESSAGE, WS_DISABLED, WS_EX_NOACTIVATE,
        },
    },
};

pub struct WindowsService {}

impl WindowsService {
    pub fn create_messaging_window(class_name: PCWSTR) -> HWND {
        let hwnd: HWND = unsafe {
            CreateWindowExW(
                WS_EX_NOACTIVATE,
                class_name,
                None,
                WS_DISABLED,
                0,
                0,
                0,
                0,
                HWND_MESSAGE,
                None,
                None,
                None,
            )
        };

        if hwnd.0 == 0 {
            let err = unsafe { GetLastError() };

            panic!(
                "Error on creating messaging window. Windows error code: {}",
                err.0
            );
        };

        hwnd
    }

    pub fn get_window_title(window_handle: HWND) -> Result<String, String> {
        let length = unsafe { GetWindowTextLengthW(window_handle) } as usize;

        if length == 0 {
            let err = unsafe { GetLastError() };

            error!(
                "Error on acquiring window title length (window might have no title or user is in desktop). Windows error code: {}",
                err.0
            );

            return Err("Could not acquire window title length".to_owned());
        }

        let mut title = vec![0; length + 1];

        let copied_length = unsafe { GetWindowTextW(window_handle, &mut title) };

        if copied_length == 0 {
            let err = unsafe { GetLastError() };

            error!(
                "Error on acquiring window title (window might have no title or user is in desktop). Windows error code: {}",
                err.0
            );

            return Err("Could not acquire window title".to_owned());
        }

        // strip null terminator
        title.pop();

        let title = String::from_utf16_lossy(&title);

        Ok(title)
    }

    pub fn get_process_id(window_handle: HWND) -> Result<u32, String> {
        let process_id_container: Option<*mut u32> = Some(&mut 0);

        let result = unsafe { GetWindowThreadProcessId(window_handle, process_id_container) };

        if result == 0 {
            let err = unsafe { GetLastError() };

            error!(
                "Error on acquiring process id. Windows error code: {}",
                err.0
            );

            return Err("Could not acquire process id".to_owned());
        }

        let pid = unsafe { *process_id_container.unwrap() };

        Ok(pid)
    }

    pub fn get_process_handle(pid: u32) -> Result<HANDLE, String> {
        let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) };

        if handle.is_err() {
            let err = unsafe { GetLastError() };

            error!(
                "Error on acquiring process handle with OpenProcess. Windows error code: {}",
                err.0
            );

            return Err("Could not acquire process handle".to_owned());
        };

        let handle = handle.unwrap();

        Ok(handle)
    }

    pub fn get_process_executable_path(process_handle: HANDLE) -> Result<String, String> {
        let mut path = vec![0; MAX_PATH as usize];

        let path_length = unsafe { GetModuleFileNameExW(process_handle, None, &mut path) };

        if path_length == 0 {
            let err = unsafe { GetLastError() };

            error!(
                "Error on retrieving process executable path. Win error code: {}",
                err.0
            );

            return Err("Could not find application executable path".to_owned());
        };

        let path = &path[0..path_length as usize];

        let path = String::from_utf16_lossy(&path);

        Ok(path)
    }

    pub fn get_app_title(path: String) -> Result<String, String> {
        let path_ref = &HSTRING::from(path);

        let size = unsafe { GetFileVersionInfoSizeW(path_ref, None) };

        if size == 0 {
            let err = unsafe { GetLastError() };

            error!(
                "Error on retrieving size of file version info. Win error code: {}",
                err.0
            );

            return Err("Error on retrieving size of file version info".to_owned());
        };

        let mut info_buffer = vec![0u8; size as usize];

        let info_result = unsafe {
            GetFileVersionInfoW(
                path_ref,
                0,
                size,
                info_buffer.as_mut_ptr() as *mut std::ffi::c_void,
            )
        }
        .as_bool();

        if !info_result {
            let err = unsafe { GetLastError() };

            error!(
                "Error on retrieving file version info. Win error code: {}",
                err.0
            );

            return Err("Error on retrieving file version info".to_owned());
        };

        let mut result_ptr = null_mut();
        let mut result_len = 0;

        let success = unsafe {
            VerQueryValueW(
                info_buffer.as_ptr() as *const std::ffi::c_void,
                h!("\\StringFileInfo\\040904B0\\FileDescription"),
                &mut result_ptr,
                &mut result_len,
            )
        }
        .as_bool();

        if !success {
            let err = unsafe { GetLastError() };

            error!(
                "Error on retrieving application title. Win error code: {}",
                err.0
            );

            return Err("Error on retrieving application title".to_owned());
        };

        let title = unsafe { slice::from_raw_parts(result_ptr as *const u16, result_len as usize) };

        let mut title = String::from_utf16_lossy(title);

        // Remove trailing null-terminator
        title.pop();

        Ok(title)
    }

    pub fn get_startup_timestamp() -> DateTime<Utc> {
        let uptime = unsafe { GetTickCount64() };

        let timestamp: i64 = Utc::now().timestamp_millis() - uptime as i64;

        NaiveDateTime::from_timestamp_millis(timestamp)
            .unwrap()
            .and_utc()
    }
}
