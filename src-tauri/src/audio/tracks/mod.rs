use std::{ffi::OsStr, path::PathBuf};

use lofty::{read_from_path, Accessor, AudioFile, TaggedFileExt};
use serde_json::{json, Value};
use tauri::command;
use walkdir::WalkDir;

#[command]
pub fn fetch_tracks(path: PathBuf) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let mut tracks: Vec<Value> = Vec::new();

    for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            println!("Found file: {:?}", file.path());
            let file_name = file
                .path()
                .file_name()
                .unwrap_or(&OsStr::new("Unknown"))
                .to_str()
                .unwrap_or("Unknown");
            let tagged_file = read_from_path(file.path()).expect("Failed to read file");
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

    Ok(tracks)
}
