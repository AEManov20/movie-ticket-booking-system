-- Your SQL goes here

CREATE TABLE IF NOT EXISTS halls (
    "id" INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    "number" INT NOT NULL,
    theatre_id INTEGER NOT NULL REFERENCES theatres("id"),
    seat_data JSON NOT NULL
);
