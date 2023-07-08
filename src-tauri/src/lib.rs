#[cfg(mobile)]
mod mobile;

#[cfg(mobile)]
pub use mobile::*;

use tauri::{path::BaseDirectory, App, Manager};
use tauri_plugin_fs::FsExt;

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
            .plugin(tauri_plugin_os::init())
            .plugin(tauri_plugin_store::Builder::default().build())
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

                // Listen for client mounted event
                app.listen_global("mounted", move |_event| {
                    println!("Front has mounted.");
                });

                Ok(())
            })
            .run(tauri::generate_context!())
            .expect("Horizon has crashed!");
    }
}
