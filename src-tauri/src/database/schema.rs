// @generated automatically by Diesel CLI.

diesel::table! {
    file (id) {
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
        file_id -> Nullable<Integer>,
        play_order -> Integer,
    }
}

diesel::joinable!(queue -> file (file_id));

diesel::allow_tables_to_appear_in_same_query!(
    file,
    queue,
);
