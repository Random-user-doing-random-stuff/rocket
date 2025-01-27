-- Your SQL goes here
CREATE TABLE IF NOT EXISTS comments (
    id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    word_id INTEGER NOT NULL,
    comment TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT comments_pk PRIMARY KEY (id),
    CONSTRAINT comments_user_id_fk FOREIGN KEY (user_id) REFERENCES users (id) ON UPDATE NO ACTION ON DELETE CASCADE,
    CONSTRAINT comments_word_id_fk FOREIGN KEY (word_id) REFERENCES words (id) ON UPDATE NO ACTION ON DELETE CASCADE
);
