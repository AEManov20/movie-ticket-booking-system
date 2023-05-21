-- This file should undo anything in `up.sql`

ALTER TABLE IF EXISTS email_confirmations DROP CONSTRAINT IF EXISTS fk_email_confirmations_users;
DROP TABLE IF EXISTS email_confirmations;
