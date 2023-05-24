use rodio::cpal::{self, traits::HostTrait};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::read;
use std::io::Cursor;
use tauri::command;

#[command]
pub async fn play_file(path: String) {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device found");
    let (_stream, stream_handle) =
        OutputStream::try_from_device(&device).expect("Could not play from audio device");

    let file = read(path).expect("Could not read from file");
    let audio = Decoder::new(Cursor::new(file)).expect("Failed to decode data from file");

    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.set_volume(0.5);

    sink.append(audio);
    sink.sleep_until_end();

    println!("Finished playing file!");
}
