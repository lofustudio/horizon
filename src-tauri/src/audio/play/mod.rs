use diesel::{delete, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use rodio::queue::{queue, SourcesQueueOutput};
use rodio::{Device, OutputStream, Sample, Sink, Source};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, Wry};
use tokio::sync::mpsc::Receiver;
use tokio::sync::RwLock;
use crate::database::DbConnection;
use crate::database::schema::queue;

pub struct Playback {
    sink: Arc<RwLock<Sink>>,
}

pub fn setup(app: AppHandle<Wry>) {
    todo!("Playback setup")
}

/// Spawning this function will set up a receiver for changing output device.
async fn device(app: AppHandle<Wry>, mut rx: Receiver<Device>) {
    // Mutable stream to prevent drop
    let mut stream = OutputStream::try_default().expect("No output devices found");

    while let Some(device) = rx.recv().await {
        // TODO: move the current playing source to the new device
        // Create stream and sink from provided device
        stream = OutputStream::try_from_device(&device).expect("Failed to create device stream");
        let sink = Sink::try_new(&stream.1).expect("Failed to use stream");

        // If sink is managed, edit; create if not
        let state = app.try_state::<Playback>();
        if let Some(state) = state {
            *state.sink.write().await = sink;
        } else {
            app.manage(RwLock::new(sink));
        }
    }
}

/// Spawning this will play the queued music.
async fn play(app: AppHandle<Wry>) {
    use crate::database::models::Queue;
    use crate::database::schema::queue::dsl;

    let playing: i32 = 0;

    loop {
        let db_state = app.state::<DbConnection>();
        let mut conn = db_state.db.lock().await;

        let sink_state = app.state::<Playback>();
        let sink = sink_state.sink.read().await;

        let result = dsl::queue
            .select(Queue::as_select())
            .order(dsl::play_order)
            .limit(1)
            .first(conn.deref_mut())
            .optional()
            .expect("Failed to select from database");

        match result {
            Some(data) => {
                // TODO: implement play to Queue and Library
            },
            None => {
                // TODO: No more queued songs.
                // Clear the queue.
                delete(queue::table)
                    .execute(conn.deref_mut())
                    .expect("Failed to clear queue");
            }
        }
        todo!("read nth entry from queue database. if it exists, play, and sleep until it ends, while also being able to seek or skip.");
    }
}
