use std::fs::read;
use std::io::Cursor;
use std::mem::ManuallyDrop;
use rodio::cpal::{self, traits::HostTrait};
use rodio::{Decoder, Device, OutputStream, Sink};
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::thread::spawn;
use tauri::{AppHandle, command, Manager, State, Wry};
use serde::Serialize;

#[derive(Serialize)]
pub struct PlaybackState {
    #[serde(skip_serializing)]
    sink: Option<Sink>,
    #[serde(skip_serializing)]
    sender: Sender<Device>,
}

impl PlaybackState {
    fn new(sender: Sender<Device>) -> Self {
        Self {
            sink: None,
            sender
        }
    }

    pub fn setup(app: AppHandle<Wry>) {
        let (sender, rx) = channel::<Device>();
        app.manage(Mutex::new(PlaybackState::new(sender.clone())));

        spawn(move || {
            // Prevent stream being dropped at the end of loop
            #[allow(unused_assignments)]
            let mut stream = OutputStream::try_default().expect("No output devices found");

            while let Ok(message) = rx.recv() {
                stream = OutputStream::try_from_device(&message).expect("Failed to use provided Device");
                let sink = Sink::try_new(&stream.1).unwrap();
                let state_mutex = app.state::<Mutex<PlaybackState>>();
                let mut state = state_mutex.lock().unwrap();
                state.sink = Some(sink);
            }
        });

        let host = cpal::default_host();
        let default = host.default_output_device().expect("No output device available");
        sender.send(default).expect("Failed to send Device");
    }

    pub fn change(&self, device: Device) {
        self.sender.send(device).expect("Failed to send Device");
    }
}

#[command]
pub async fn play_file(path: String, audio: State<'_, Mutex<PlaybackState>>) -> Result<(), ()> {
    let file = read(path).expect("Could not read from file");
    let cursor = Decoder::new(Cursor::new(file)).expect("Failed to decode data from file");

    audio.lock().unwrap().sink.as_ref().unwrap().append(cursor);

    Ok(())
}

#[command]
pub async fn pause(audio: State<'_, Mutex<PlaybackState>>) -> Result<(), ()> {
    let playback = audio.lock().unwrap();
    let sink = playback.sink.as_ref().unwrap();
    if sink.is_paused() { sink.play() } else { sink.pause() }

    Ok(())
}
