mod file;
pub mod files;

use crate::audio::discover::files::save_files;
use crate::database::DbConnection;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager, Wry};
use tauri_plugin_fs::FsExt;

pub async fn setup(app: &AppHandle<Wry>) {
    // Allow fs to access system audio directory
    let audio_path = app.path().resolve("Horizon", BaseDirectory::Audio).unwrap();
    if std::fs::read_dir(&audio_path).err().is_some() {
        std::fs::create_dir(&audio_path).expect("Could not create audio directory")
    }
    app.fs_scope().allow_directory(&audio_path, true).unwrap();

    // Save list of files to database
    let db = app.state::<DbConnection>();
    save_files(&audio_path, db).await;
}
