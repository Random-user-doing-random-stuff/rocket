-- Drop all triggers first to avoid dependency issues
DROP TRIGGER IF EXISTS prevent_duplicate_word_tag_trigger ON word_tags;
DROP TRIGGER IF EXISTS prevent_inactive_word_tag_deletion_trigger ON word_tags;
DROP TRIGGER IF EXISTS prevent_inactive_word_tag_insertion_trigger ON word_tags;
DROP TRIGGER IF EXISTS check_word_tag_relationship_trigger ON word_tags;

-- Drop all functions
DROP FUNCTION IF EXISTS prevent_duplicate_word_tag;
DROP FUNCTION IF EXISTS prevent_inactive_word_tag_deletion;
DROP FUNCTION IF EXISTS prevent_inactive_word_tag_insertion;
DROP FUNCTION IF EXISTS check_word_tag_relationship;

-- Drop the word_tags table
DROP TABLE IF EXISTS word_tags;
