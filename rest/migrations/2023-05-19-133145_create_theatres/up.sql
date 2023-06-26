-- Your SQL goes here

CREATE TABLE IF NOT EXISTS theatres (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "name" VARCHAR(50) NOT NULL,
    location_lat FLOAT NOT NULL,
    location_lon FLOAT NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    logo_image_url VARCHAR(2048),
    cover_image_url VARCHAR(2048)
);
