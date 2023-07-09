use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;
use rodio::Sink;
use crate::audio::source::MusicSource;

pub struct MusicQueue {
    queue: Arc<Mutex<Vec<Box<dyn MusicSource + Send + Sync>>>>
}

impl MusicQueue {
    pub fn new(sink: Arc<Sink>) -> Self {
        let queue: Arc<Mutex<Vec<Box<dyn MusicSource + Send + Sync>>>> = Arc::new(Mutex::new(Vec::new()));
        let clone = queue.clone();

        spawn(move || {
            loop {
                if let Some(element) = {
                    if let Ok(mut guard) = clone.lock() {
                        if !guard.is_empty() {
                            Some(guard.remove(0))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } {
                    element.play(&sink)
                } else {
                    // Wait for 1 sec if queue is empty
                    // TODO: find a better way to tell if queue isn't empty
                    sleep(Duration::from_secs(1))
                };
            }
        });

        MusicQueue {queue}
    }

    pub fn push(&self, source: Box<dyn MusicSource + Send + Sync>) {
        self.queue.lock().unwrap().push(source);
    }
}