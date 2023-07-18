mod file;
pub mod files;

use crate::audio::discover::files::{clear, save_files};
use crate::database::DbConnection;
use log::debug;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager, Wry};
use tauri_plugin_fs::FsExt;

/// Set up discovery and add files to the database
pub async fn setup(app: &AppHandle<Wry>) {
    debug!("Setting up discovery");

    // Clear the database first
    let db = app.state::<DbConnection>();
    clear(&db).await;

    // Allow fs to access system audio directory
    let audio_path = app.path().resolve("Horizon", BaseDirectory::Audio).unwrap();
    if std::fs::read_dir(&audio_path).err().is_some() {
        std::fs::create_dir(&audio_path).expect("Could not create audio directory")
    }
    app.fs_scope().allow_directory(&audio_path, true).unwrap();

    // TODO: make sure the database is created
    // Save list of files to database
    save_files(&audio_path, db).await;
}
