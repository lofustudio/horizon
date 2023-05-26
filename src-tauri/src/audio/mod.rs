mod thread;

use rodio::cpal::{self, traits::HostTrait};
use rodio::{Device, OutputStream, Sink};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;
use std::thread::spawn;
use tauri::{command, State};
use crate::audio::thread::Command;
use crate::audio::thread::{PlayFile, Pause};

pub struct Audio {
    // Consider sync_channel
    sender: Mutex<Sender<Command>>
}

impl Audio {
    fn new(device: Device) -> Self {
        let (tx, rx): (Sender<Command>, Receiver<Command>) = channel();

        let _join_handle = spawn(move || {
            let (_stream, stream_handle) =
                OutputStream::try_from_device(&device).expect("Could not play from audio device");

            let sink = Sink::try_new(&stream_handle).unwrap();

            while let Ok(cmd) = rx.recv() {
                match cmd {
                    Command::PlayFile(t) => t.run(&sink),
                    Command::Pause(t) => t.run(&sink)
                }
            }
        });

        Self {
            sender: Mutex::new(tx)
        }
    }
}

impl Default for Audio {
    fn default() -> Self {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("No output device found");

        Audio::new(device)
    }
}

#[command]
pub async fn play_file(path: String, audio: State<'_, Audio>) -> Result<(), ()> {
    let x = Command::PlayFile(PlayFile::new(path));
    audio.sender.lock().unwrap().send(x).expect("Failed to send file");

    Ok(())
}

#[command]
pub async fn pause(audio: State<'_, Audio>) -> Result<(), ()> {
    let x = Command::Pause(Pause::new());
    audio.sender.lock().unwrap().send(x).expect("Failed to pause");

    Ok(())
}
