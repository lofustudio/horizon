use crate::audio::play::Playback;
use crate::database::models::File;
use futures::executor::block_on;
use rodio::Decoder;
use std::io::Cursor;
use tokio::fs::read;
use tokio::task::spawn_blocking;

impl File {
    /// Play the file. This function will await until it has finished playing.
    pub async fn play(&self, playback: &Playback) {
        let file = read(&self.path).await.expect("Failed to read from file");
        let cursor = Decoder::new(Cursor::new(file)).expect("Failed to decode data from file");

        let arc = &playback.sink;
        let sink = arc.read().await;
        sink.append(cursor);

        let clone = arc.clone();
        spawn_blocking(move || {
            block_on(clone.read()).sleep_until_end();
        })
        .await
        .expect("Blocking thread crashed");
    }
}
