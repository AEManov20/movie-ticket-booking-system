-- Your SQL goes here

CREATE TABLE IF NOT EXISTS halls (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    theatre_id UUID NOT NULL REFERENCES theatres("id"),
    "name" VARCHAR(50) NOT NULL,
    seat_data JSON NOT NULL
);
