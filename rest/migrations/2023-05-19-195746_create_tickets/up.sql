-- Your SQL goes here

CREATE TABLE IF NOT EXISTS tickets (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_user_id UUID NOT NULL REFERENCES users("id"),
    theatre_screening_id UUID NOT NULL REFERENCES theatre_screenings("id"),
    ticket_type_id UUID NOT NULL,
    issuer_user_id UUID NOT NULL,
    seat_row INTEGER NOT NULL,
    seat_column INTEGER NOT NULL,
    issued_at TIMESTAMP NOT NULL DEFAULT now(),
    expires_at TIMESTAMP NOT NULL,
    used BOOLEAN NOT NULL
);
