-- This file should undo anything in `up.sql`

ALTER TABLE IF EXISTS theatre_movies DROP CONSTRAINT IF EXISTS fk_theatre_movies_halls;
ALTER TABLE IF EXISTS theatre_movies DROP CONSTRAINT IF EXISTS fk_theatre_movies_movies;
DROP TABLE IF EXISTS theatre_movies;