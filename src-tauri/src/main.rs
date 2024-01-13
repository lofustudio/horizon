// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod audio;
mod commands;

#[tokio::main]
async fn main() {
    // Initialize logger
    let mut builder = pretty_env_logger::formatted_timed_builder();
    builder.filter_level(log::LevelFilter::Trace);
    builder.parse_env("RUST_LOG");
    builder.init();

    // Start Tauri application
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::utils::check_app_dir,
            commands::audio::play
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
