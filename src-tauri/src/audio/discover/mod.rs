mod file;
pub mod files;

use crate::audio::discover::files::save_files;
use crate::database::DbConnection;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager, Wry};
use tauri_plugin_fs::FsExt;

/// Set up discovery and add files to the database
pub async fn setup(app: &AppHandle<Wry>) {
    debug!("Setting up discovery");
    // Allow fs to access system audio directory
    let audio_path = app.path().resolve("Horizon", BaseDirectory::Audio).unwrap();
    if std::fs::read_dir(&audio_path).err().is_some() {
        std::fs::create_dir(&audio_path).expect("Could not create audio directory")
    }
    app.fs_scope().allow_directory(&audio_path, true).unwrap();

    // TODO: make sure the database is created
    // Save list of files to database
    let db = app.state::<DbConnection>();
    save_files(&audio_path, db).await;
}
