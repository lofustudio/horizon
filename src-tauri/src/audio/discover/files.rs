use diesel::{insert_into, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use serde_json::{json, Value};
use std::ops::DerefMut;
use std::path::Path;
use std::sync::Mutex;
use tauri::{command, State};
use walkdir::WalkDir;

pub fn save_tracks(path: &Path, db: State<Mutex<SqliteConnection>>) {
    use crate::database::schema::library;
    use crate::database::models::NewLibrary;

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

#[command]
pub fn fetch_tracks(db: State<Mutex<SqliteConnection>>) -> Result<Vec<Value>, ()> {
    use crate::database::schema::library::dsl::library;
    use crate::database::models::Library;

    // Fetch library entries from database
    let mut conn = db.lock().unwrap();
    let lib = library.select(Library::as_select()).load(conn.deref_mut()).expect("Failed to select from database");
    let test = lib.iter().map(|x| json!(x)).collect();
    Ok(test)
}