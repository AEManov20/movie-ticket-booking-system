-- Your SQL goes here

CREATE TABLE IF NOT EXISTS halls (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "number" INT NOT NULL,
    theatre_id UUID NOT NULL REFERENCES theatres("id"),
    seat_data JSON NOT NULL
);
