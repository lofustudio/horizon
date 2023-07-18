pub mod command;
mod file;
mod queue;

use crate::database::DbConnection;
use futures::executor::block_on;
use log::debug;
use rodio::cpal::traits::HostTrait;
use rodio::{cpal, Device, OutputStream, Sink};
use std::ops::Deref;
use std::sync::Arc;
use tauri::{AppHandle, Manager, Wry};
use tokio::spawn;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::RwLock;
use tokio::task::spawn_blocking;

pub struct Playback {
    pub sink: Arc<RwLock<Sink>>,
    pub playing: RwLock<bool>,
    pub play: Sender<()>,
}

impl Playback {
    /// This function will set up the Playback state.
    pub async fn setup(app: &AppHandle<Wry>) {
        debug!("Setting up the playback!");

        let clone = app.clone();
        let (device_tx, device_rx) = channel::<Device>(10);

        spawn_blocking(|| Playback::device(clone, device_rx));

        let host = cpal::default_host();
        let default = host
            .default_output_device()
            .expect("No output device available");
        device_tx
            .send(default)
            .await
            .expect("Failed to send Device");
    }

    /// Spawning this blocking function will add Playback to states and set up a receiver for changing the output device.
    fn device(app: AppHandle<Wry>, mut device_rx: Receiver<Device>) {
        debug!("Setting up device changing!");

        // Mutable stream to prevent drop
        #[allow(unused_assignments)]
        let mut stream = OutputStream::try_default().expect("No output devices found");

        while let Some(device) = block_on(device_rx.recv()) {
            // TODO: move the current playing source to the new device
            // Create stream and sink from provided device
            stream =
                OutputStream::try_from_device(&device).expect("Failed to create device stream");
            let sink = Sink::try_new(&stream.1).expect("Failed to use stream");

            let state = app.try_state::<Playback>();
            if let Some(state) = state {
                // Playback is already managed, edit
                *block_on(state.sink.write()) = sink;
            } else {
                // Create a new Playback, manage
                let (play_tx, play_rx) = channel(1);
                app.manage(Playback {
                    sink: Arc::new(RwLock::new(sink)),
                    playing: RwLock::new(false),
                    play: play_tx,
                });
                spawn(Playback::player(app.clone(), play_rx));
            }
        }
    }

    /// Spawning this will play the queued music.
    async fn player(app: AppHandle<Wry>, mut play: Receiver<()>) {
        debug!("Starting the player!");
        use crate::database::models::Queue;

        let mut playing: i32 = 0;

        loop {
            let db_state = app.state::<DbConnection>();

            // Select first entry with play_order equal to playing.
            let result = Queue::next(db_state.deref(), &playing).await;

            match result {
                Some(data) => {
                    debug!("Playing queued song {}", data.play_order);
                    app.emit_all("playing", data.play_order)
                        .expect("Failed to emit event");
                    // TODO: implement seeking/stopping

                    let sink_state = app.state::<Playback>();
                    playing = data.play_order + 1;
                    *sink_state.playing.write().await = true;

                    // Await until the song has finished playing.
                    data.play(db_state.deref(), sink_state.deref()).await;
                }
                None => {
                    debug!("The queue is empty.");
                    app.emit_all("stopped", ()).expect("Failed to emit event");

                    // Clear the queue.
                    Queue::clear(db_state.deref()).await;

                    let sink_state = app.state::<Playback>();

                    playing = 0;
                    *sink_state.playing.write().await = false;

                    // Wait for a message before looking for the next queued song.
                    play.recv().await;
                }
            }
        }
    }
}
