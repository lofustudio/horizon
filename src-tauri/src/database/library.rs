use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::library)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Library {
    pub id: i32,
    pub path: String
}