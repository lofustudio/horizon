use std::ops::DerefMut;
use diesel::{ExpressionMethods, insert_into, QueryDsl, RunQueryDsl, SelectableHelper};
use serde_json::{json, Value};
use tauri::{command, State};
use crate::audio::play::Playback;
use crate::database::DbConnection;
use crate::database::models::{NewQueue, Queue};
use crate::database::schema::queue;
use crate::database::schema::queue::{dsl, play_order};

#[command]
/// Tauri command to add a File to the Queue
pub async fn add_file_to_queue(db: State<'_, DbConnection>, playback: State<'_, Playback>, file: i32) -> Result<(), ()> {
    debug!("add_file_to_queue invoked");
    let mut conn = db.db.lock().await;

    // Get the biggest play_order
    let playorder = dsl::queue
        .select(play_order)
        .order_by(play_order.desc())
        .limit(1)
        .first::<i32>(conn.deref_mut())
        .expect("Failed to select play_order");

    insert_into(queue::table)
        .values(NewQueue { file_id: Some(file), play_order: playorder + 1} )
        .execute(conn.deref_mut())
        .expect("Failed to insert to Queue");

    if !(*playback.playing.read().await) {
        playback.play.send(()).await.expect("Failed to continue playing");
    }

    Ok(())
}

#[command]
/// Tauri command to return everything in the queue table as an array of json.
pub async fn fetch_queue(db: State<'_, DbConnection>) -> Result<Vec<Value>, ()> {
    debug!("fetch_queue invoked");

    // Fetch library entries from database
    let mut conn = db.db.lock().await;
    let lib = dsl::queue
        .select(Queue::as_select())
        .load(conn.deref_mut())
        .expect("Failed to select from database");

    // Return as an array of json
    let test = lib.iter().map(|x| json!(x)).collect();
    Ok(test)
}