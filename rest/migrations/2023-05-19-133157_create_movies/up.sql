-- Your SQL goes here

CREATE TABLE IF NOT EXISTS movies (
    "id" INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    "name" VARCHAR(250) NOT NULL,
    "description" TEXT NOT NULL,
    genre VARCHAR(250) NOT NULL,
    release_date DATE NOT NULL,
    "length" FLOAT NOT NULL,
    imdb_link VARCHAR(250),
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,

    UNIQUE ("name")
);
