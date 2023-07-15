use crate::database::models::NewFile;
use crate::database::DbConnection;
use diesel::{insert_into, RunQueryDsl};
use lofty::{read_from_path, Accessor, AudioFile, TaggedFileExt};
use std::ffi::OsStr;
use std::ops::DerefMut;
use std::path::PathBuf;

impl NewFile {
    /// Create and insert NewFile into the database.
    pub async fn insert(path: &PathBuf, db: &DbConnection) {
        use crate::database::schema::file;

        // TODO: this assumes the file is not already in the library
        let mut conn = db.db.lock().await;
        insert_into(file::table)
            .values(NewFile::new(path))
            .execute(conn.deref_mut())
            .expect("Could not insert into library");
    }

    /// Read the file and create a NewFile.
    fn new(path: &PathBuf) -> Self {
        let file_name = path
            .file_name()
            .unwrap_or(OsStr::new("Unknown"))
            .to_str()
            .unwrap_or("Unknown");
        let tagged_file = read_from_path(path).expect("Failed to read file");
        let tag = tagged_file.primary_tag().expect("Primary tag not found");
        let properties = tagged_file.properties();

        Self {
            path: path
                .to_str()
                .expect("Failed to unwrap path str")
                .parse()
                .unwrap(),
            title: tag.title().as_deref().unwrap_or(file_name).parse().unwrap(),
            artist: tag.artist().as_deref().map(|data| data.to_string()),
            album: tag.album().as_deref().map(|data| data.to_string()),
            genre: tag.genre().as_deref().map(|data| data.to_string()),
            duration: Some(
                i32::try_from(properties.duration().as_millis()).expect("Failed to store duration"),
            ),
        }
    }
}
