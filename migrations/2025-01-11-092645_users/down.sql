-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS update_user_timestamp ON users;

DROP FUNCTION IF EXISTS update_user_timestamp;

DROP TABLE IF EXISTS users;

DROP TYPE IF EXISTS user_role;
