use crate::database::DbConnection;
use diesel::{insert_into, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use serde_json::{json, Value};
use std::ops::DerefMut;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tauri::{command, State};
use walkdir::WalkDir;

pub async fn save_tracks(path: &Path, db: State<'_, DbConnection>) {
    use crate::database::models::NewLibrary;
    use crate::database::schema::library;

    for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            println!("Found file: {:?}", file.path());
            let new_library = NewLibrary::new(file.into_path());

            // TODO: this assumes the music is not already in the library
            let mut conn = db.db.lock().await;
            insert_into(library::table)
                .values(new_library)
                .execute(conn.deref_mut())
                .expect("Could not insert into library");
        }
    }
}

#[command]
pub async fn fetch_tracks(db: State<'_, DbConnection>) -> Result<Vec<Value>, ()> {
    use crate::database::models::Library;
    use crate::database::schema::library::dsl::library;

    // Fetch library entries from database
    let mut conn = db.db.lock().await;
    let lib = library
        .select(Library::as_select())
        .load(conn.deref_mut())
        .expect("Failed to select from database");
    let test = lib.iter().map(|x| json!(x)).collect();
    Ok(test)
}
