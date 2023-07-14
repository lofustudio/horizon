#[cfg(mobile)]
mod mobile;

#[cfg(mobile)]
pub use mobile::*;

mod audio;
mod database;

use crate::audio::discover::files::fetch_tracks;
use tauri::App;

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

    #[tokio::main]
    pub async fn run(self) {
        let setup = self.setup;
        tauri::Builder::default()
            .plugin(tauri_plugin_fs::init())
            .plugin(tauri_plugin_window::init())
            .plugin(tauri_plugin_os::init())
            // Make commands invokable from frontend
            .invoke_handler(tauri::generate_handler![fetch_tracks])
            .setup(move |app| {
                if let Some(setup) = setup {
                    (setup)(app)?;
                }

                // Set up the database first
                database::setup(app.handle());

                // Set up the backend
                audio::setup(app.handle());

                Ok(())
            })
            .run(tauri::generate_context!())
            .expect("Horizon has crashed!");
    }
}
