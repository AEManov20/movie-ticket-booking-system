-- This file should undo anything in `up.sql`

ALTER TABLE IF EXISTS theatre_permissions DROP CONSTRAINT IF EXISTS fk_theatre_permissions_users;
ALTER TABLE IF EXISTS theatre_permissions DROP CONSTRAINT IF EXISTS fk_theatre_permissions_theatres;
DROP TABLE IF EXISTS theatre_permissions;
