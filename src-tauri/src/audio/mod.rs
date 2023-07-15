pub mod discover;
mod play;

use tauri::AppHandle;

pub async fn setup(app: AppHandle) {
    // Setup file discovery first
    discover::setup(&app).await;

    // Setup playback
    play::Playback::setup(&app).await;
}
