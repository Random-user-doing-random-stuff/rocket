CREATE TYPE word_status AS ENUM ('pending', 'approved', 'rejected', 'inactive');

-- Define an ENUM for word types
CREATE TYPE word_type AS ENUM ('verb', 'noun', 'adjective', 'adverb', 'pronoun', 'preposition', 'conjunction', 'interjection', 'other');

-- Modify words table to include word_type
CREATE TABLE IF NOT EXISTS words (
    id SERIAL PRIMARY KEY,
    term VARCHAR NOT NULL UNIQUE,
    video VARCHAR(255) NOT NULL UNIQUE,
    word_type word_type NOT NULL,  -- Added word_type ENUM
    created_by INTEGER NOT NULL,
    status word_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    FOREIGN KEY (created_by) REFERENCES users (id),
    CONSTRAINT check_status CHECK (status IN ('pending', 'approved', 'rejected', 'inactive'))
);

CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_words_timestamp
BEFORE UPDATE ON words
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();


CREATE OR REPLACE FUNCTION set_deleted_at()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.status = 'inactive' THEN
        NEW.deleted_at = CURRENT_TIMESTAMP;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_deleted_at_trigger
BEFORE UPDATE ON words
FOR EACH ROW
WHEN (NEW.status = 'inactive')
EXECUTE FUNCTION set_deleted_at();


CREATE OR REPLACE FUNCTION prevent_rejected_after_approved()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.status = 'approved' AND NEW.status = 'rejected' THEN
        RAISE EXCEPTION 'Cannot change status from "approved" to "rejected"';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER prevent_rejected_after_approved_trigger
BEFORE UPDATE ON words
FOR EACH ROW
EXECUTE FUNCTION prevent_rejected_after_approved();


CREATE OR REPLACE FUNCTION enforce_unique_term_type()
RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (SELECT 1 FROM words WHERE term = NEW.term AND word_type = NEW.word_type) THEN
        RAISE EXCEPTION 'Duplicate word type for term: %', NEW.term;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER enforce_unique_term_type_trigger
BEFORE INSERT ON words
FOR EACH ROW
EXECUTE FUNCTION enforce_unique_term_type();
