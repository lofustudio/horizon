mod discover;

use tauri::{AppHandle, Wry};

pub fn setup(app: AppHandle<Wry>) {
    // Setup file discovery
    discover::setup(app);

    todo!("Thread to play music & next in queue if it is over")
}
