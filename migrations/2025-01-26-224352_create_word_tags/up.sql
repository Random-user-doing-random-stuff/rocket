-- Your SQL goes here
CREATE TABLE IF NOT EXISTS word_tags (
    word_id INTEGER,
    tag_id INTEGER,
    CONSTRAINT word_tags_pk PRIMARY KEY (word_id, tag_id),
    CONSTRAINT word_tags_word_id_fk FOREIGN KEY (word_id) REFERENCES words (id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    CONSTRAINT word_tags_tag_id_fk FOREIGN KEY (tag_id) REFERENCES tags (id) ON UPDATE NO ACTION ON DELETE NO ACTION
);
