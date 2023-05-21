-- This file should undo anything in `up.sql`

ALTER TABLE IF EXISTS external_credentials DROP CONSTRAINT IF EXISTS fk_external_credentials_users;
DROP TABLE IF EXISTS external_credentials;
