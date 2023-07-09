use diesel::{Connection, QueryDsl, SqliteConnection};
use tauri::{AppHandle, Wry};

mod library;
mod schema;

pub fn setup(app: AppHandle<Wry>) {
    use schema::library::dsl::*;

    let conn = SqliteConnection::establish("audio.db")
        .unwrap_or_else(|_| panic!("Error connecting to the db"));
}