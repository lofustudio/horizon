use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Identifiable, Serialize)]
#[diesel(table_name = crate::database::schema::file)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct File {
    pub id: i32,
    pub path: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub genre: String,
    pub duration: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::file)]
pub struct NewFile {
    pub path: String,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub duration: Option<i32>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize)]
#[diesel(belongs_to(File))]
#[diesel(table_name = crate::database::schema::queue)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Queue {
    pub id: i32,
    pub file_id: Option<i32>,
    pub play_order: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::queue)]
pub struct NewQueue {
    pub file_id: Option<i32>,
    pub play_order: i32,
}
