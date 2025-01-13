-- Your SQL goes here

CREATE TYPE user_role AS ENUM ('user', 'professor', 'admin');

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR,
    email VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    phone_number VARCHAR,
    role user_role NOT NULL DEFAULT 'user',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP,
    password_reset_token VARCHAR,
    password_reset_token_expiry TIMESTAMP,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    CONSTRAINT check_role CHECK (role IN ('user', 'professor', 'admin'))
);

CREATE OR REPLACE FUNCTION update_user_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.last_updated = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_user_timestamp
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_user_timestamp();
