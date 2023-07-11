use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::database::schema::library)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Library {
    pub id: i32,
    pub path: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub genre: String,
    pub duration: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::library)]
pub struct NewLibrary {
    pub path: String,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub duration: Option<i32>,
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Library))]
#[diesel(table_name = crate::database::schema::queue)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Queue {
    pub id: i32,
    pub library_id: i32,
}
