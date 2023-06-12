-- Your SQL goes here

CREATE TABLE IF NOT EXISTS halls (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    theatre_id UUID NOT NULL REFERENCES theatres("id"),
    "name" VARCHAR(250) NOT NULL,
    price_increase FLOAT NOT NULL DEFAULT 0,
    seat_data JSON NOT NULL
);
