use serde::Serialize;
use rodio::{Sink, Source};

pub(crate) mod file;

pub trait MusicSource {
    fn play(&self, sink: &Sink);
    // TODO: implement play starting at certain duration
}

#[derive(Serialize)]
pub struct TrackInfo {
    title: String,
    artist: String,
    album: String,
    genre: String,
    duration: u128
}