-- Your SQL goes here

CREATE TABLE IF NOT EXISTS tickets (
    "id" INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    owner_user_id INTEGER NOT NULL REFERENCES users("id"),
    theatre_movie_id INTEGER NOT NULL REFERENCES theatre_movies("id"),
    ticket_type_id INTEGER NOT NULL,
    issuer_user_id INTEGER NOT NULL,
    seat_row INTEGER NOT NULL,
    seat_column INTEGER NOT NULL,
    issued_at TIMESTAMP NOT NULL DEFAULT now(),
    expires_at TIMESTAMP NOT NULL,
    used BOOLEAN NOT NULL
);
