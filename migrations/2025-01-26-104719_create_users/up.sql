-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL,
    username TEXT NOT NULL,
    first_name TEXT,
    last_name TEXT,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    phone_number TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    permissions SMALLINT NOT NULL CHECK (permissions >= 0) DEFAULT 1, --bitwise permissions (2bytes)
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    password_reset_token TEXT,
    password_reset_token_expires_at TIMESTAMP,
    CONSTRAINT valid_permissions CHECK (permissions = (permissions & 63)),
    CONSTRAINT users_pk PRIMARY KEY (id),
    CONSTRAINT users_username_unique UNIQUE (username)
);
