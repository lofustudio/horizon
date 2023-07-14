// @generated automatically by Diesel CLI.

diesel::table! {
    library (id) {
        id -> Integer,
        path -> Text,
        title -> Text,
        artist -> Text,
        album -> Text,
        genre -> Text,
        duration -> Integer,
    }
}

diesel::table! {
    queue (id) {
        id -> Integer,
        library_id -> Integer,
        play_order -> Integer,
    }
}

diesel::joinable!(queue -> library (library_id));

diesel::allow_tables_to_appear_in_same_query!(library, queue,);
