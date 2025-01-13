-- Your SQL goes here
CREATE TABLE IF NOT EXISTS word_tags (
    PRIMARY KEY (word_id, tag_id),
    word_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    FOREIGN KEY (word_id) REFERENCES words (id),
    FOREIGN KEY (tag_id) REFERENCES tags (id)
);
