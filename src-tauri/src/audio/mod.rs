mod discover;

use tauri::{AppHandle, Wry};

pub fn setup(app: AppHandle<Wry>) {
    // Setup file discovery
    discover::setup(app);
}