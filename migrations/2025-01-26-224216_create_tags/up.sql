-- Your SQL goes here
CREATE TABLE IF NOT EXISTS tags (
    id INTEGER NOT NULL,
    name TEXT NOT NULL,
    CONSTRAINT tags_pk PRIMARY KEY (id),
    CONSTRAINT tags_name_unique UNIQUE (name)
);
