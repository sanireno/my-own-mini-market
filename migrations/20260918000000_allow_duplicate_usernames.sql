-- username is a display name, not a login identifier.
-- Email remains unique and is used to sign in.
ALTER TABLE users
DROP CONSTRAINT IF EXISTS users_username_key;
