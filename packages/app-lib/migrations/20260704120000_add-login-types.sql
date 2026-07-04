-- Add login_type and server_url columns to minecraft_users
-- to support offline and yggdrasil (third-party) login methods alongside existing Microsoft SISU login.
-- Existing rows are treated as 'microsoft' by default.

ALTER TABLE minecraft_users ADD COLUMN login_type TEXT NOT NULL DEFAULT 'microsoft';
ALTER TABLE minecraft_users ADD COLUMN server_url TEXT;

-- Drop the unique index on active column to allow multiple accounts of different types
-- (the application logic still keeps only one active account at a time via upsert)
DROP INDEX IF EXISTS minecraft_users_active;
