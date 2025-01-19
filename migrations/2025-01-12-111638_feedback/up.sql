CREATE TYPE feedback_type AS ENUM ('suggestion', 'bug_report', 'comment');

CREATE TABLE IF NOT EXISTS feedbacks (
    id SERIAL PRIMARY KEY,
    word_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    feedback TEXT NOT NULL,
    feedback_type feedback_type NOT NULL DEFAULT 'comment', -- Added type
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    response TEXT,  -- Admin can respond to feedback
    response_timestamp TIMESTAMP,
    FOREIGN KEY (word_id) REFERENCES words (id),
    FOREIGN KEY (user_id) REFERENCES users (id)
);


CREATE OR REPLACE FUNCTION set_response_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.response IS NOT NULL AND OLD.response IS NULL THEN
        NEW.response_timestamp = CURRENT_TIMESTAMP;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_response_timestamp_trigger
BEFORE UPDATE ON feedbacks
FOR EACH ROW
WHEN (NEW.response IS NOT NULL AND OLD.response IS NULL)
EXECUTE FUNCTION set_response_timestamp();

CREATE OR REPLACE FUNCTION check_word_user_exists()
RETURNS TRIGGER AS $$
BEGIN
    -- Check if word_id exists in the words table
    IF NOT EXISTS (SELECT 1 FROM words WHERE id = NEW.word_id) THEN
        RAISE EXCEPTION 'Invalid word_id: %', NEW.word_id;
    END IF;

    -- Check if user_id exists in the users table
    IF NOT EXISTS (SELECT 1 FROM users WHERE id = NEW.user_id) THEN
        RAISE EXCEPTION 'Invalid user_id: %', NEW.user_id;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_word_user_exists_trigger
BEFORE INSERT ON feedbacks
FOR EACH ROW
EXECUTE FUNCTION check_word_user_exists();

CREATE OR REPLACE FUNCTION prevent_empty_feedback()
RETURNS TRIGGER AS $$
BEGIN
    IF TRIM(NEW.feedback) = '' THEN
        RAISE EXCEPTION 'Feedback cannot be empty';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER prevent_empty_feedback_trigger
BEFORE INSERT ON feedbacks
FOR EACH ROW
EXECUTE FUNCTION prevent_empty_feedback();

CREATE OR REPLACE FUNCTION set_feedback_type_based_on_keywords()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.feedback ILIKE '%bug%' OR NEW.feedback ILIKE '%error%' OR NEW.feedback ILIKE '%problem%' THEN
        NEW.feedback_type = 'bug report';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_feedback_type_based_on_keywords_trigger
BEFORE INSERT ON feedbacks
FOR EACH ROW
EXECUTE FUNCTION set_feedback_type_based_on_keywords();

CREATE OR REPLACE FUNCTION set_feedback_type_to_suggestion()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.feedback ILIKE '%improve%' OR NEW.feedback ILIKE '%suggest%' OR NEW.feedback ILIKE '%recommend%' THEN
        NEW.feedback_type = 'suggestion';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_feedback_type_to_suggestion_trigger
BEFORE INSERT ON feedbacks
FOR EACH ROW
EXECUTE FUNCTION set_feedback_type_to_suggestion();
