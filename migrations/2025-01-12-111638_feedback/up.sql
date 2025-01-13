-- Your SQL goes here
CREATE TABLE IF NOT EXISTS feedbacks (
    id SERIAL PRIMARY KEY,
    word_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    feedback TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (word_id) REFERENCES words (id),
    FOREIGN KEY (user_id) REFERENCES users (id)
);
