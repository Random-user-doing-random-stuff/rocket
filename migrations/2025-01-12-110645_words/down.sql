-- Drop all triggers first to avoid dependency issues
DROP TRIGGER IF EXISTS update_words_timestamp ON words;
DROP TRIGGER IF EXISTS set_deleted_at_trigger ON words;
DROP TRIGGER IF EXISTS prevent_rejected_after_approved_trigger ON words;
DROP TRIGGER IF EXISTS enforce_unique_term_type_trigger ON words;

-- Drop all functions
DROP FUNCTION IF EXISTS update_timestamp;
DROP FUNCTION IF EXISTS set_deleted_at;
DROP FUNCTION IF EXISTS prevent_rejected_after_approved;
DROP FUNCTION IF EXISTS enforce_unique_term_type;

-- Drop the words table
DROP TABLE IF EXISTS words;

-- Drop the ENUM types
DROP TYPE IF EXISTS word_status;
DROP TYPE IF EXISTS word_type;
