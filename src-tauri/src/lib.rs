mod audio;
#[cfg(mobile)]
mod mobile;

use audio::{play_file, pause};
use tauri::{path::BaseDirectory, App, Manager};
use tauri_plugin_fs::FsExt;
#[cfg(mobile)]
pub use mobile::*;
use crate::audio::PlaybackState;

//noinspection RsWrongGenericArgumentsNumber
pub type SetupHook = Box<dyn FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send>;

#[derive(Default)]
pub struct AppBuilder {
    setup: Option<SetupHook>,
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

                let audio_path = app.path().resolve("", BaseDirectory::Audio)?;

                if std::fs::read_dir(&audio_path).err().is_some() { std::fs::create_dir(&audio_path).expect("Could not create audio directory") }

                app.fs_scope().allow_directory(audio_path, true)?;

                PlaybackState::setup(app.handle());

                Ok(())
            })
            .run(tauri::generate_context!())
            .expect("Horizon has crashed!");
    }
}