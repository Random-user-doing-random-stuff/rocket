-- Your SQL goes here
CREATE TABLE IF NOT EXISTS words (
    id SERIAL PRIMARY KEY,
    term VARCHAR NOT NULL UNIQUE,
    video VARCHAR NOT NULL UNIQUE,
    created_by INTEGER NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (created_by) REFERENCES users (id),
    CONSTRAINT check_status CHECK (
        status IN ('pending', 'approved', 'rejected', 'inactive')
    )
);
