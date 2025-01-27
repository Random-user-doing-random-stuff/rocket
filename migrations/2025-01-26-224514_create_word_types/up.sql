-- Your SQL goes here
CREATE TABLE IF NOT EXISTS word_type (
    id INTEGER NOT NULL,
    type_name TEXT NOT NULL,
    CONSTRAINT word_type_pk PRIMARY KEY (id),
    CONSTRAINT word_type_type_unique UNIQUE (type_name)
);
