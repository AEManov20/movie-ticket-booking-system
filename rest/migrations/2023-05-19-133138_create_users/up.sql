-- Your SQL goes here

CREATE TABLE IF NOT EXISTS users (
    "id" INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    email VARCHAR(150) NOT NULL,
    username VARCHAR(50) NOT NULL,
    -- TODO: migrate to CHAR(150) since only ascii characters are allowed
    password_hash VARCHAR(150),
    is_super_user BOOLEAN NOT NULL DEFAULT FALSE,
    is_activated BOOLEAN NOT NULL DEFAULT FALSE,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,

    UNIQUE (email),
    UNIQUE (username)
);
