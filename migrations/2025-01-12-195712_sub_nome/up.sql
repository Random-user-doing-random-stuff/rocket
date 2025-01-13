-- Rename the `name` column to `first_name`
ALTER TABLE users
RENAME COLUMN name TO first_name;

-- Add a new `last_name` column with a default value
ALTER TABLE users
ADD COLUMN last_name VARCHAR NOT NULL DEFAULT '';

-- Add an optional `phone_number` column
ALTER TABLE users
ADD COLUMN phone_number VARCHAR;

-- Add a required `birthdate` column (NOT NULL)
ALTER TABLE users
ADD COLUMN birthdate DATE NOT NULL DEFAULT '2000-01-01';
