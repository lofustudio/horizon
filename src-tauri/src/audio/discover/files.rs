use crate::database::models::NewLibrary;
use diesel::{insert_into, RunQueryDsl, SqliteConnection};
use serde_json::Value;
use std::ops::DerefMut;
use std::path::Path;
use std::sync::Mutex;
use tauri::{command, State};
use walkdir::WalkDir;

#[command]
pub fn fetch_tracks(path: &Path) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let mut tracks: Vec<Value> = Vec::new();

    todo!("Columns in Library to Vec, and then return")
}

pub fn save_tracks(path: &Path, db: State<Mutex<SqliteConnection>>) {
    use crate::database::schema::library;

    for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            println!("Found file: {:?}", file.path());
            let new_library = NewLibrary::new(file.into_path());

            let mut conn = db.lock().unwrap();
            // TODO: this assumes the music is not already in the library
            let _ = insert_into(library::table)
                .values(&new_library)
                .execute(conn.deref_mut());
        }
    }
}
