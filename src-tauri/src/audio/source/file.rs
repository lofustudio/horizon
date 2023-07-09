use std::fs::read;
use std::io::Cursor;
use lofty::{Accessor, AudioFile, read_from_path, TaggedFileExt};
use rodio::{Decoder, Sink};
use serde::Serialize;
use crate::audio::source::{MusicSource, TrackInfo};

#[derive(Serialize)]
pub struct FileSource {
    path: String,
    pub info: TrackInfo
}

impl FileSource {
    pub fn new(link: String) -> Self {
        let tagged_file = read_from_path(&link).expect("Failed to read from path");
        // TODO: If primary tag is not found, iterate through the tags to find a suitable one
        let tag = tagged_file.primary_tag().expect("Primary tag not found");
        let properties = tagged_file.properties();

        let info = TrackInfo {
            title: tag.title().as_deref().unwrap_or("Unknown").to_string(),
            artist: tag.artist().as_deref().unwrap_or("Unknown").to_string(),
            album: tag.album().as_deref().unwrap_or("Unknown").to_string(),
            genre: tag.genre().as_deref().unwrap_or("Unknown").to_string(),
            duration: properties.duration().as_millis()
        };

        FileSource { path: link, info }
    }
}

impl MusicSource for FileSource {
    fn play(&self, sink: &Sink) {
        let file = read(&self.path).expect("Could not read from file");
        let cursor = Decoder::new(Cursor::new(file)).expect("Failed to decode data from file");
        sink.append(cursor);
    }
}
