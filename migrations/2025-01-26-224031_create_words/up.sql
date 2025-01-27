-- Your SQL goes here
CREATE TYPE word_status AS ENUM ('draft', 'review', 'approved', 'archived');

CREATE TABLE IF NOT EXISTS words (
    id INTEGER NOT NULL,
    word TEXT NOT NULL,
    definition TEXT NOT NULL,
    video_url TEXT NOT NULL,
    created_by INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    status word_status DEFAULT 'draft',
    CONSTRAINT words_pk PRIMARY KEY (id),
    CONSTRAINT words_created_by_fk FOREIGN KEY (created_by) REFERENCES users (id) ON UPDATE NO ACTION ON DELETE NO ACTION
);
