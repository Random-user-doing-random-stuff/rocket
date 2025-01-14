-- Drop all triggers first to avoid dependency issues
DROP TRIGGER IF EXISTS set_response_timestamp_trigger ON feedbacks;
DROP TRIGGER IF EXISTS check_word_user_exists_trigger ON feedbacks;
DROP TRIGGER IF EXISTS prevent_empty_feedback_trigger ON feedbacks;
DROP TRIGGER IF EXISTS set_feedback_type_based_on_keywords_trigger ON feedbacks;
DROP TRIGGER IF EXISTS set_feedback_type_to_suggestion_trigger ON feedbacks;

-- Drop all functions
DROP FUNCTION IF EXISTS set_response_timestamp;
DROP FUNCTION IF EXISTS check_word_user_exists;
DROP FUNCTION IF EXISTS prevent_empty_feedback;
DROP FUNCTION IF EXISTS set_feedback_type_based_on_keywords;
DROP FUNCTION IF EXISTS set_feedback_type_to_suggestion;

-- Drop the feedbacks table
DROP TABLE IF EXISTS feedbacks;

-- Drop the ENUM type
DROP TYPE IF EXISTS feedback_type;
