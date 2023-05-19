use rodio::cpal::{self, source::Source, traits::HostTrait, Decoder, OutputStream};
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;
use tauri::api::file::read_binary;
use tauri::command;

#[command]
async fn play_file(path: String) {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device found");
    let (_stream, stream_handle) =
        OutputStream::try_from_device(&device).expect("Could not play from audio device");

    let file = read_binary(path).expect("Could not read from file");
    let audio = Decoder::new(Cursor::new(file)).expect("Failed to decode data from file");

    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.set_volume(0.5);

    sink.append(audio);
    sink.sleep_until_end();

    println!("Finished playing file!");
    return;
}
