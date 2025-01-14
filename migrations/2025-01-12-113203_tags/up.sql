CREATE TABLE IF NOT EXISTS tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    description TEXT,  -- Optional tag description
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE OR REPLACE FUNCTION prevent_empty_tag_name()
RETURNS TRIGGER AS $$
BEGIN
    IF TRIM(NEW.name) = '' THEN
        RAISE EXCEPTION 'Tag name cannot be empty or whitespace';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER prevent_empty_tag_name_trigger
BEFORE INSERT OR UPDATE ON tags
FOR EACH ROW
EXECUTE FUNCTION prevent_empty_tag_name();

CREATE OR REPLACE FUNCTION set_created_at_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.created_at IS NULL THEN
        NEW.created_at = CURRENT_TIMESTAMP;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_created_at_timestamp_trigger
BEFORE INSERT ON tags
FOR EACH ROW
EXECUTE FUNCTION set_created_at_timestamp();

CREATE OR REPLACE FUNCTION set_is_active_default()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.is_active IS NULL THEN
        NEW.is_active = TRUE;  -- Defaults to TRUE if NULL
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_is_active_default_trigger
BEFORE INSERT OR UPDATE ON tags
FOR EACH ROW
EXECUTE FUNCTION set_is_active_default();

CREATE OR REPLACE FUNCTION prevent_tag_name_update()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.name IS DISTINCT FROM NEW.name THEN
        RAISE EXCEPTION 'Tag name cannot be changed after creation';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER prevent_tag_name_update_trigger
BEFORE UPDATE ON tags
FOR EACH ROW
EXECUTE FUNCTION prevent_tag_name_update();

CREATE OR REPLACE FUNCTION enforce_unique_tag_name()
RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (SELECT 1 FROM tags WHERE name = NEW.name) THEN
        RAISE EXCEPTION 'Tag name % already exists.', NEW.name;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER enforce_unique_tag_name_trigger
BEFORE INSERT OR UPDATE ON tags
FOR EACH ROW
EXECUTE FUNCTION enforce_unique_tag_name();
