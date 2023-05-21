-- This file should undo anything in `up.sql`

ALTER TABLE IF EXISTS tickets DROP CONSTRAINT IF EXISTS fk_tickets_theatre_movies;
ALTER TABLE IF EXISTS tickets DROP CONSTRAINT IF EXISTS fk_tickets_users;
DROP TABLE IF EXISTS tickets;
