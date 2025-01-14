CREATE TABLE IF NOT EXISTS word_tags (
    word_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (word_id, tag_id),
    FOREIGN KEY (word_id) REFERENCES words(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

CREATE OR REPLACE FUNCTION prevent_duplicate_word_tag()
RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (SELECT 1 FROM word_tags WHERE word_id = NEW.word_id AND tag_id = NEW.tag_id) THEN
        RAISE EXCEPTION 'The combination of word_id % and tag_id % already exists in word_tags.', NEW.word_id, NEW.tag_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER prevent_duplicate_word_tag_trigger
BEFORE INSERT ON word_tags
FOR EACH ROW
EXECUTE FUNCTION prevent_duplicate_word_tag();

CREATE OR REPLACE FUNCTION prevent_inactive_word_tag_deletion()
RETURNS TRIGGER AS $$
BEGIN
    -- Check if either word or tag is inactive
    IF EXISTS (SELECT 1 FROM words WHERE id = OLD.word_id AND is_active = FALSE) OR
       EXISTS (SELECT 1 FROM tags WHERE id = OLD.tag_id AND is_active = FALSE) THEN
        RAISE EXCEPTION 'Cannot delete word_tag relationship because the word or tag is inactive.';
    END IF;
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER prevent_inactive_word_tag_deletion_trigger
BEFORE DELETE ON word_tags
FOR EACH ROW
EXECUTE FUNCTION prevent_inactive_word_tag_deletion();

CREATE OR REPLACE FUNCTION prevent_inactive_word_tag_insertion()
RETURNS TRIGGER AS $$
BEGIN
    -- Check if the word or tag is inactive
    IF EXISTS (SELECT 1 FROM words WHERE id = NEW.word_id AND is_active = FALSE) OR
       EXISTS (SELECT 1 FROM tags WHERE id = NEW.tag_id AND is_active = FALSE) THEN
        RAISE EXCEPTION 'Cannot create word_tag relationship because the word or tag is inactive.';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER prevent_inactive_word_tag_insertion_trigger
BEFORE INSERT ON word_tags
FOR EACH ROW
EXECUTE FUNCTION prevent_inactive_word_tag_insertion();

CREATE OR REPLACE FUNCTION check_word_tag_relationship()
RETURNS TRIGGER AS $$
BEGIN
    -- Check if the word exists
    IF NOT EXISTS (SELECT 1 FROM words WHERE id = NEW.word_id) THEN
        RAISE EXCEPTION 'Word with ID % does not exist.', NEW.word_id;
    END IF;
    
    -- Check if the tag exists
    IF NOT EXISTS (SELECT 1 FROM tags WHERE id = NEW.tag_id) THEN
        RAISE EXCEPTION 'Tag with ID % does not exist.', NEW.tag_id;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_word_tag_relationship_trigger
BEFORE INSERT ON word_tags
FOR EACH ROW
EXECUTE FUNCTION check_word_tag_relationship();
