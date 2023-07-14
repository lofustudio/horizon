CREATE TABLE queue (
    id INTEGER PRIMARY KEY NOT NULL,
    library_id INTEGER NOT NULL,
    play_order INTEGER NOT NULL,
    FOREIGN KEY (library_id) REFERENCES library(id)
)