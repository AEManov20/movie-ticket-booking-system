-- Your SQL goes here

CREATE TABLE IF NOT EXISTS theatres (
    "id" INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    "name" VARCHAR(50) NOT NULL,
    location_lat FLOAT NOT NULL,
    location_lon FLOAT NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE
);
