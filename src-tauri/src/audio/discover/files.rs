use crate::database::DbConnection;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use serde_json::{json, Value};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use tauri::{command, State};
use walkdir::WalkDir;

/// Crawls the given directory to find files and saves detected files into the database.
pub async fn save_files(path: &Path, db: State<'_, DbConnection>) {
    use crate::database::models::NewFile;

    for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            // TODO: is the file a music file or something else?
            debug!("Found file: {:?}", file.path());
            NewFile::insert(&file.into_path(), db.deref()).await;
        }
    }
}

#[command]
/// Tauri command to return everything in the file table as an array of json.
pub async fn fetch_files(db: State<'_, DbConnection>) -> Result<Vec<Value>, ()> {
    use crate::database::models::File;
    use crate::database::schema::file::dsl::file;

    // Fetch library entries from database
    let mut conn = db.db.lock().await;
    let lib = file
        .select(File::as_select())
        .load(conn.deref_mut())
        .expect("Failed to select from database");

    // Return as an array of json
    let test = lib.iter().map(|x| json!(x)).collect();
    Ok(test)
}
