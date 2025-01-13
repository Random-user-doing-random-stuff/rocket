-- This file should undo anything in `up.sql`
ALTER TABLE users
ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP;
