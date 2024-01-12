// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use tauri::Manager as _;

#[tokio::main]
async fn main() {
    // Initialize logger
    pretty_env_logger::init();

    // Start Tauri application
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}