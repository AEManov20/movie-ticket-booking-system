-- Your SQL goes here

CREATE TABLE IF NOT EXISTS ticket_types (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "type" VARCHAR(50) NOT NULL,
    movie_type VARCHAR(50) NOT NULL,
    "description" VARCHAR(300),
    theatre_id UUID NOT NULL REFERENCES theatres("id"),

    -- NOTE: this is compliant to the ISO 4217 standard, which specifies three-letter ("Alpha-3") codes for currencies worldwide
    currency VARCHAR(3) NOT NULL,
    price FLOAT NOT NULL
);

ALTER TABLE IF EXISTS tickets ADD CONSTRAINT tickets_ticket_types_fkey FOREIGN KEY(ticket_type_id) REFERENCES ticket_types("id");
