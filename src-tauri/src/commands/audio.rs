use tauri::command;

use crate::audio::decode::decode_mp3;
use crate::audio::output::AudioOutput;

/// Test code for the audio output.
#[command]
pub async fn play() {
    // TODO: only create one instance of the audio output
    let mut output = AudioOutput::try_new().unwrap();

    let _sample_rate = output.config.sample_rate.0 as f32;

    let mut buf: Vec<f32> = Vec::new();
    // this blocks until all the data is decoded
    let result = decode_mp3("./song.mp3", &mut buf).await;

    if result.is_err() {
        error!("{}", result.err().unwrap());
        trace!("buf len: {}", buf.len());
    }

    for (_, sample) in buf.iter().enumerate() {
        if output.buf_w.push(*sample).is_err() {
            std::thread::sleep(std::time::Duration::from_millis(25));
        }
    }
}
