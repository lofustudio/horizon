use crate::audio::play::Playback;
use crate::database::models::{File, Queue};
use crate::database::schema::queue::play_order;
use crate::database::DbConnection;
use diesel::{
    delete, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
};
use log::debug;
use std::ops::DerefMut;

impl Queue {
    /// Gets the next entry from the Queue (play_order >= playing)
    pub async fn next(db: &DbConnection, playing: &i32) -> Option<Queue> {
        use crate::database::schema::queue::dsl;
        let mut conn = db.db.lock().await;

        // Select first entry with play_order equal (or greater than) to playing.
        dsl::queue
            .select(Queue::as_select())
            .filter(dsl::play_order.ge(playing))
            .order_by(play_order.asc())
            .first(conn.deref_mut())
            .optional()
            .expect("Failed to select from database")
    }

    /// Clears the queue database
    pub async fn clear(db: &DbConnection) {
        use crate::database::schema::queue::table;
        let mut conn = db.db.lock().await;

        delete(table)
            .execute(conn.deref_mut())
            .expect("Failed to clear queue");
    }

    /// Detects the source of the Queue and appends it to the playback sink.
    /// This awaits until the playback has finished.
    pub async fn play(&self, db: &DbConnection, sink: &Playback) {
        let mut conn = db.db.lock().await;

        // Queue is from a file
        if let Some(file) = self.file_id {
            use crate::database::schema::file::dsl;

            let file = dsl::file
                .select(File::as_select())
                .find(file)
                .get_result(conn.deref_mut())
                .expect("Failed to get file from relation");

            debug!("Playing {}", file.title);
            file.play(sink).await;
        }
    }
}
