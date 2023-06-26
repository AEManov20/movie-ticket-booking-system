-- Your SQL goes here

CREATE TABLE IF NOT EXISTS users (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    email VARCHAR(150) NOT NULL UNIQUE,
    username VARCHAR(50) NOT NULL UNIQUE,
    password_hash VARCHAR(150),
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    is_super_user BOOLEAN NOT NULL DEFAULT FALSE,
    is_activated BOOLEAN NOT NULL DEFAULT FALSE,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    profile_picture_url VARCHAR(2048)
);
