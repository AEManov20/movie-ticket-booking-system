-- This file should undo anything in `up.sql`

ALTER TABLE IF EXISTS halls DROP CONSTRAINT IF EXISTS fk_halls_theatres;
DROP TABLE IF EXISTS halls;