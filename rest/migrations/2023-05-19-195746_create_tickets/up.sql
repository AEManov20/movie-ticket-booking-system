-- Your SQL goes here

CREATE TABLE IF NOT EXISTS tickets (
    "id" INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    owner_user_id INTEGER NOT NULL,
    theatre_movie_id INTEGER NOT NULL,
    ticket_type_id INTEGER NOT NULL,
    issuer_user_id INTEGER,
    seat_row INTEGER NOT NULL,
    seat_column INTEGER NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    used BOOLEAN NOT NULL,

    CONSTRAINT fk_tickets_theatre_movies FOREIGN KEY(theatre_movie_id) REFERENCES theatre_movies("id"),
    CONSTRAINT fk_tickets_users FOREIGN KEY(owner_user_id) REFERENCES users("id")
);
