-- Your SQL goes here

CREATE TABLE IF NOT EXISTS movies (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "name" VARCHAR(250) NOT NULL UNIQUE,
    "description" TEXT NOT NULL,
    genre VARCHAR(250) NOT NULL,
    release_date DATE NOT NULL,
    "length" FLOAT NOT NULL,
    "votes" INTEGER NOT NULL DEFAULT 0,
    imdb_link VARCHAR(250),
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE
);