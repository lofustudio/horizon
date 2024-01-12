use dirs::audio_dir;
use tauri::command;

#[command]
/// Finds or creates the user's music directory.
pub async fn check_app_dir() -> Result<String, String> {
    if let Some(path) = audio_dir() {
        // Music directory exists, return path
        Ok(path.to_str().unwrap().to_string())
    } else {
        // TODO: Create music directory
        Err("Could not find or create music directory.".to_string())
    }
}
