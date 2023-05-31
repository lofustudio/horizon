#[cfg(mobile)]
mod mobile;

use lofty::{read_from_path, Accessor, AudioFile, TaggedFileExt};
#[cfg(mobile)]
pub use mobile::*;
use serde_json::{json, Value};
use std::ffi::OsStr;

mod audio;

use crate::audio::PlaybackState;
use audio::{pause, play_file};
use tauri::{path::BaseDirectory, App, Manager};
use tauri_plugin_fs::FsExt;
use walkdir::WalkDir;

//noinspection RsWrongGenericArgumentsNumber
pub type SetupHook = Box<dyn FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send>;

#[derive(Default)]
pub struct AppBuilder {
    setup: Option<SetupHook>,
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    //noinspection RsWrongGenericArgumentsNumber
    #[must_use]
    pub fn setup<F>(mut self, setup: F) -> Self
    where
        F: FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send + 'static,
    {
        self.setup.replace(Box::new(setup));
        self
    }

    pub fn run(self) {
        let setup = self.setup;
        tauri::Builder::default()
            .plugin(tauri_plugin_fs::init())
            .plugin(tauri_plugin_window::init())
            .invoke_handler(tauri::generate_handler![play_file, pause])
            .setup(move |app| {
                if let Some(setup) = setup {
                    (setup)(app)?;
                }

                // Allow fs to access system audio directory
                let audio_path = app.path().resolve("Horizon", BaseDirectory::Audio)?;
                if std::fs::read_dir(&audio_path).err().is_some() {
                    std::fs::create_dir(&audio_path).expect("Could not create audio directory")
                }
                app.fs_scope().allow_directory(&audio_path, true)?;

                // Setup playback state
                PlaybackState::setup(app.handle());

                // Listen for client mounted event
                let app_handle = app.handle();
                app.listen_global("mounted", move |event| {
                    println!("Front has mounted. Fetching tracks...");
                    let mut tracks: Vec<Value> = Vec::new();

                    for file in WalkDir::new(audio_path.clone())
                        .into_iter()
                        .filter_map(|file| file.ok())
                    {
                        if file.metadata().unwrap().is_file() {
                            println!("Found file: {:?}", file.path());
                            let file_name = file
                                .path()
                                .file_name()
                                .unwrap_or(&OsStr::new("Unknown"))
                                .to_str()
                                .unwrap_or("Unknown");
                            let tagged_file =
                                read_from_path(file.path()).expect("Failed to read file");
                            let tag = tagged_file.primary_tag().expect("Primary tag not found");
                            let properties = tagged_file.properties();

                            let track = json!({
                                "title": tag.title().as_deref().unwrap_or(file_name).to_string(),
                                "artist": tag.artist().as_deref().unwrap_or("Unknown").to_string(),
                                "album": tag.album().as_deref().unwrap_or("Unknown").to_string(),
                                "genre": tag.genre().as_deref().unwrap_or("Unknown").to_string(),
                                "path": file.path().to_str().unwrap().to_string(),
                                "duration": properties.duration().as_millis(),
                            });

                            tracks.push(track);
                        }
                    }

                    // Send tracks to client
                    app_handle
                        .emit_all("tracks", tracks)
                        .expect("Could not emit tracks");
                });

                Ok(())
            })
            .run(tauri::generate_context!())
            .expect("Horizon has crashed!");
    }
}
