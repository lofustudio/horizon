use diesel::{Connection, SqliteConnection};
use std::future::Future;
use std::ops::DerefMut;
use std::sync::Arc;
use tauri::{AppHandle, Manager, Wry};
use tokio::sync::{Mutex, MutexGuard};

pub mod insert;
pub mod models;
pub mod schema;

pub struct DbConnection {
    pub db: Arc<Mutex<SqliteConnection>>,
}

impl DbConnection {
    pub async fn setup(app: AppHandle<Wry>) {
        // Connect to the sqlite database
        // TODO: set location for audio.db
        let conn = SqliteConnection::establish("audio.db")
            .unwrap_or_else(|_| panic!("Error connecting to the db"));

        // Add Mutex<SqliteConnection> to states
        app.manage(DbConnection {
            db: Arc::new(Mutex::new(conn)),
        });
    }
}
