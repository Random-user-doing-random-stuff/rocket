-- Drop all triggers first to avoid dependency issues
DROP TRIGGER IF EXISTS update_user_timestamp ON users;
DROP TRIGGER IF EXISTS update_last_login_trigger ON users;
DROP TRIGGER IF EXISTS prevent_delete_admin_or_professor_trigger ON users;
DROP TRIGGER IF EXISTS deactivate_user_if_token_expired_trigger ON users;
DROP TRIGGER IF EXISTS enforce_unique_email_trigger ON users;

-- Drop all functions
DROP FUNCTION IF EXISTS update_user_timestamp;
DROP FUNCTION IF EXISTS update_last_login;
DROP FUNCTION IF EXISTS prevent_delete_admin_or_professor;
DROP FUNCTION IF EXISTS deactivate_user_if_token_expired;
DROP FUNCTION IF EXISTS enforce_unique_email;

-- Drop the users table
DROP TABLE IF EXISTS users;

-- Drop the ENUM type
DROP TYPE IF EXISTS user_role;
