use rodio::cpal::{self, traits::HostTrait};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::read;
use std::io::Cursor;
use std::sync::Mutex;
use tauri::{command, State};

pub struct Audio {
    sink: Mutex<Sink>
}

impl Default for Audio {
    fn default() -> Self {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("No output device found");
        let (_stream, stream_handle) =
            OutputStream::try_from_device(&device).expect("Could not play from audio device");

        Self {
            sink: Mutex::new(Sink::try_new(&stream_handle).unwrap()),
        }
    }
}

#[command]
pub async fn play_file<'r>(path: String, audio: State<'r, Audio>) -> Result<(), ()> {
    let file = read(path).expect("Could not read from file");
    let cursor = Decoder::new(Cursor::new(file)).expect("Failed to decode data from file");

    let sink = audio.sink.lock().unwrap();
    sink.set_volume(0.5);
    sink.append(cursor);
    // sink.sleep_until_end();

    println!("Playing file!");
    Ok(())
}

#[command]
pub async fn pause<'r>(audio: State<'r, Audio>) -> Result<(), ()> {
    let sink = audio.sink.lock().unwrap();
    if sink.is_paused() { sink.pause() } else { sink.play() }

    println!("Paused!");
    Ok(())
}