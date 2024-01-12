// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use audio::output::AudioOutput;

mod audio;
mod commands;

#[tokio::main]
async fn main() {
    // Initialize logger
    let mut builder = pretty_env_logger::formatted_builder();
    builder.filter_level(log::LevelFilter::Trace);
    builder.parse_env("RUST_LOG");
    builder.init();

    // Start Tauri application
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::utils::check_app_dir])
        .setup(|_app| {
            // This is test code for the audio output!
            // TODO: Remove this when we have a real audio engine
            let mut output = AudioOutput::try_new().unwrap();

            let sample_rate = output.config.sample_rate.0 as f32;

            // Produce a sinusoid of 128 Hz.
            let mut sample_clock = 0f32;
            let mut next_value = move || {
                sample_clock = (sample_clock + 1.0) % sample_rate;
                (sample_clock * 128.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
            };

            loop {
                if output.buf_w.push(next_value()).is_err() {
                    std::thread::sleep(std::time::Duration::from_millis(5));
                }
            }

            // Allow unreachable code for now
            #[allow(unreachable_code)]
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
