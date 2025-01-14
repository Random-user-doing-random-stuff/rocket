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

CREATE OR REPLACE FUNCTION update_last_login()
RETURNS TRIGGER AS $$
BEGIN
    NEW.last_login = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_last_login_trigger
BEFORE UPDATE ON users
FOR EACH ROW
WHEN (NEW.last_login IS DISTINCT FROM OLD.last_login)
EXECUTE FUNCTION update_last_login();

CREATE OR REPLACE FUNCTION prevent_delete_admin_or_professor()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.role IN ('admin', 'professor') THEN
        RAISE EXCEPTION 'Cannot delete a user with role %', OLD.role;
    END IF;
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER prevent_delete_admin_or_professor_trigger
BEFORE DELETE ON users
FOR EACH ROW
EXECUTE FUNCTION prevent_delete_admin_or_professor();

CREATE OR REPLACE FUNCTION deactivate_user_if_token_expired()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.password_reset_token_expiry < CURRENT_TIMESTAMP THEN
        NEW.is_active = FALSE;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER deactivate_user_if_token_expired_trigger
BEFORE UPDATE ON users
FOR EACH ROW
WHEN (NEW.password_reset_token_expiry IS NOT NULL AND NEW.password_reset_token_expiry < CURRENT_TIMESTAMP)
EXECUTE FUNCTION deactivate_user_if_token_expired();

CREATE OR REPLACE FUNCTION enforce_unique_email()
RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (SELECT 1 FROM users WHERE email = NEW.email) THEN
        RAISE EXCEPTION 'Email address % is already in use', NEW.email;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER enforce_unique_email_trigger
BEFORE INSERT OR UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION enforce_unique_email();

