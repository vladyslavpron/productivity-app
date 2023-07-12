use std::backtrace::Backtrace;
use std::panic;

use dotenv::dotenv;

use tokio::join;

extern crate dotenv;

#[macro_use]
extern crate rocket;

mod database;
mod entity;
mod server;
mod service;

// TODO: server security, unauthorized access might be dangerous
// TODO: custom default port
// TODO: make config from env variables, throw error at start if there are some missing
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    setup_logger().unwrap();

    panic::set_hook(Box::new(|info| {
        let stacktrace = Backtrace::force_capture();
        println!("Got panic. @info:{}\n@stackTrace:{}", info, stacktrace);
        std::process::abort();
    }));

    let db = database::setup_database().await;

    info!("Database setup completed");

    let mut service = service::Service::new(db.clone()).await;
    let service_job = service.spin_loop();

    let server = rocket::build()
        .manage(db.clone())
        .mount("/", routes![server::serve_files])
        .mount(
            "/api",
            routes![
                server::get_events,
                server::get_current_session,
                server::get_current_session_statistics,
                server::get_current_session_events
            ],
        )
        .launch();

    info!("Joining service and server");

    let (_, server) = join!(service_job, server);

    server.unwrap();

    info!("Service and server finished their work, shutting down...");

    db.close().await.unwrap();

    Ok(())
}

fn drop_rocket(meta: &log::Metadata) -> bool {
    let name = meta.target();
    if name.starts_with("rocket") || name.eq("_") {
        return false;
    }
    true
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .filter(drop_rocket)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
