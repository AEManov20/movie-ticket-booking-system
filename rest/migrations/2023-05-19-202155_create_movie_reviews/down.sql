-- This file should undo anything in `up.sql`

ALTER TABLE IF EXISTS movie_reviews DROP CONSTRAINT IF EXISTS fk_movie_reviews_users;
ALTER TABLE IF EXISTS movie_reviews DROP CONSTRAINT IF EXISTS fk_movie_reviews_movies;
DROP TABLE IF EXISTS movie_reviews;
