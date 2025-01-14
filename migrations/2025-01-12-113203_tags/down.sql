-- Drop all triggers first to avoid dependency issues
DROP TRIGGER IF EXISTS prevent_empty_tag_name_trigger ON tags;
DROP TRIGGER IF EXISTS set_created_at_timestamp_trigger ON tags;
DROP TRIGGER IF EXISTS set_is_active_default_trigger ON tags;
DROP TRIGGER IF EXISTS prevent_tag_name_update_trigger ON tags;
DROP TRIGGER IF EXISTS enforce_unique_tag_name_trigger ON tags;

-- Drop all functions
DROP FUNCTION IF EXISTS prevent_empty_tag_name;
DROP FUNCTION IF EXISTS set_created_at_timestamp;
DROP FUNCTION IF EXISTS set_is_active_default;
DROP FUNCTION IF EXISTS prevent_tag_name_update;
DROP FUNCTION IF EXISTS enforce_unique_tag_name;

-- Drop the tags table
DROP TABLE IF EXISTS tags;
