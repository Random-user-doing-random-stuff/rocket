-- Rename the `first_name` column back to `name`
ALTER TABLE users
RENAME COLUMN first_name TO name;

-- Drop the `last_name` column
ALTER TABLE users
DROP COLUMN last_name;

-- Drop the `phone_number` column
ALTER TABLE users
DROP COLUMN phone_number;

-- Drop the `birthdate` column
ALTER TABLE users
DROP COLUMN birthdate;
