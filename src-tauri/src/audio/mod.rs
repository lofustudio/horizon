mod source;
pub(crate) mod tracks;
mod queue;

use rodio::cpal::{self, traits::HostTrait};
use rodio::{Device, OutputStream, Sink};
use serde::Serialize;
use serde_json::{json, Value};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;
use tauri::{command, AppHandle, Manager, State, Wry};
use crate::audio::queue::MusicQueue;
use crate::audio::source::{MusicSource, file::FileSource};

#[derive(Serialize)]
pub struct PlaybackState {
    #[serde(skip_serializing)]
    queue: Option<MusicQueue>,
    #[serde(skip_serializing)]
    sink: Option<Arc<Sink>>,
    #[serde(skip_serializing)]
    sender: Sender<Device>,
}

impl PlaybackState {
    fn new(sender: Sender<Device>) -> Self {
        Self { queue: None, sink: None, sender }
    }

    pub fn setup(app: AppHandle<Wry>) {
        let (sender, rx) = channel::<Device>();
        app.manage(Mutex::new(PlaybackState::new(sender.clone())));
        let clone = app.clone();

        // TODO: Consider Tokio
        spawn(move || {
            // Prevent stream being dropped at the end of loop
            #[allow(unused_assignments)]
            let mut stream = OutputStream::try_default().expect("No output devices found");

            // TODO: move the current playing source to the new device
            while let Ok(message) = rx.recv() {
                stream =
                    OutputStream::try_from_device(&message).expect("Failed to use provided Device");
                let sink = Sink::try_new(&stream.1).unwrap();
                let state_mutex = clone.state::<Mutex<PlaybackState>>();
                let mut state = state_mutex.lock().unwrap();
                state.sink = Some(Arc::new(sink));
            }
        });

        let host = cpal::default_host();
        let default = host
            .default_output_device()
            .expect("No output device available");
        sender.send(default).expect("Failed to send Device");

        // TODO: find a better way to tell if queue has been built
        sleep(Duration::from_secs(1));

        let state_mutex = app.state::<Mutex<PlaybackState>>();
        let mut state = state_mutex.lock().unwrap();
        let queue = MusicQueue::new(state.sink.clone().unwrap());
        state.queue = Some(queue);
    }

    pub fn change(&self, device: Device) {
        self.sender.send(device).expect("Failed to send Device");
    }
}

#[command]
pub async fn play_file(path: String, audio: State<'_, Mutex<PlaybackState>>) -> Result<Value, ()> {
    let track = FileSource::new(path);
    track.play(audio.lock().unwrap().sink.as_ref().unwrap());

    Ok(json!(track.info))
}

#[command]
pub async fn queue_file(path: String, audio: State<'_, Mutex<PlaybackState>>) -> Result<Value, ()> {
    let track = FileSource::new(path);
    let mut guard = audio.lock().unwrap();
    let queue = guard.queue.as_mut().unwrap();
    let info = json!(track.info);
    queue.push(Box::new(track));

    Ok(info)
}

#[command]
pub async fn toggle_pause(audio: State<'_, Mutex<PlaybackState>>) -> Result<bool, ()> {
    let playback = audio.lock().unwrap();
    let sink = playback.sink.as_ref().unwrap();
    if sink.is_paused() {
        sink.play()
    } else {
        sink.pause()
    }

    Ok(sink.is_paused())
}
