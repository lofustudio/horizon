use diesel::{Connection, SqliteConnection};
use std::sync::Mutex;
use tauri::{AppHandle, Manager, Wry};

mod insert;
pub(crate) mod models;
pub(crate) mod schema;

pub fn setup(app: AppHandle<Wry>) {
    // Connect to the sqlite database
    let conn = SqliteConnection::establish("audio.db")
        .unwrap_or_else(|_| panic!("Error connecting to the db"));

    // Add Mutex<SqliteConnection> to states
    app.manage(Mutex::new(conn));
}
