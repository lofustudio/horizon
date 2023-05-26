use std::fs::read;
use std::io::{Cursor};
use rodio::{Decoder, Sink};

pub enum Command {
    PlayFile(PlayFile),
    Pause(Pause)
}

pub struct PlayFile {
    path: String
}

impl PlayFile {
    pub fn new(path: String) -> Self {
        Self {
            path
        }
    }
    pub fn run(&self, sink: &Sink) {
        let file = read(&self.path).expect("Could not read from file");
        let cursor = Decoder::new(Cursor::new(file)).expect("Failed to decode data from file");

        sink.append(cursor);
    }
}

pub struct Pause {}

impl Pause {
    pub fn new() -> Self {
        Self {}
    }
    pub fn run(&self, sink: &Sink) {
        if sink.is_paused() { sink.play() } else { sink.pause() }
    }
}