-- Your SQL goes here
CREATE TABLE IF NOT EXISTS word_has_type (
    type_id INTEGER NOT NULL,
    word_id INTEGER NOT NULL,
    CONSTRAINT word_has_type_pk PRIMARY KEY (type_id, word_id),
    CONSTRAINT word_has_type_type_id_fk FOREIGN KEY (type_id) REFERENCES word_type (id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    CONSTRAINT word_has_type_word_id_fk FOREIGN KEY (word_id) REFERENCES words (id) ON UPDATE NO ACTION ON DELETE NO ACTION
);
