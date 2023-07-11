pub(crate) mod files;

use crate::audio::discover::files::{fetch_tracks, save_tracks};
use diesel::SqliteConnection;
use std::sync::Mutex;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager, Wry};
use tauri_plugin_fs::FsExt;

pub fn setup(app: AppHandle<Wry>) {
    // Allow fs to access system audio directory
    let audio_path = app.path().resolve("Horizon", BaseDirectory::Audio).unwrap();
    if std::fs::read_dir(&audio_path).err().is_some() {
        std::fs::create_dir(&audio_path).expect("Could not create audio directory")
    }
    app.fs_scope().allow_directory(&audio_path, true).unwrap();

    // Save list of files to database
    let db = app.state::<Mutex<SqliteConnection>>();
    save_tracks(&audio_path, db);

    // Listen for client mounted event
    let clone = app.clone();
    app.listen_global("mounted", move |_| {
        println!("Front has mounted. Fetching tracks and status...");

        // Send tracks to client
        let tracks = fetch_tracks(&audio_path).expect("Failed to fetch tracks");
        clone
            .emit_all("tracks", tracks)
            .expect("Could not emit tracks");
    });
}
