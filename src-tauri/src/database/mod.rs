use diesel::{Connection, SqliteConnection};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

pub mod models;
pub mod schema;

pub struct DbConnection {
    pub db: Arc<Mutex<SqliteConnection>>,
}

impl DbConnection {
    pub fn setup(app: &AppHandle) {
        debug!("Connecting to the database!");
        // Connect to the sqlite database
        // TODO: set location for audio.db, create if doesn't exist
        let conn = SqliteConnection::establish("audio.db")
            .unwrap_or_else(|_| panic!("Error connecting to the db"));

        // Add Mutex<SqliteConnection> to states
        app.manage(DbConnection {
            db: Arc::new(Mutex::new(conn)),
        });
    }
}
