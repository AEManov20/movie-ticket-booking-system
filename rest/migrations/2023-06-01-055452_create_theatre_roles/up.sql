-- Your SQL goes here

CREATE TABLE IF NOT EXISTS theatre_roles (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "name" VARCHAR(150) UNIQUE NOT NULL
);