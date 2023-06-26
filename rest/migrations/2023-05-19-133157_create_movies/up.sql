-- Your SQL goes here

CREATE TABLE IF NOT EXISTS movies (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "name" VARCHAR(250) NOT NULL UNIQUE,
    "description" TEXT NOT NULL,
    genre VARCHAR(250) NOT NULL,
    release_date DATE NOT NULL,
    "length" FLOAT NOT NULL,
    imdb_link VARCHAR(2048),
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    poster_image_url VARCHAR(2048)
);
