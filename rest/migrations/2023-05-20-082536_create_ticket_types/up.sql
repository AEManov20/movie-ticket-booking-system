-- Your SQL goes here

CREATE TABLE IF NOT EXISTS ticket_types (
    "id" INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    "type" VARCHAR(50) NOT NULL,
    movie_type VARCHAR(50) NOT NULL,
    "description" VARCHAR(300),
    theatre_id INTEGER NOT NULL,

    -- NOTE: this is compliant to the ISO 4217 standard, which specifies three-letter ("Alpha-3") codes for currencies worldwide
    currency VARCHAR(3) NOT NULL,
    price FLOAT NOT NULL,

    CONSTRAINT fk_ticket_types_theatres FOREIGN KEY(theatre_id) REFERENCES theatres("id")
);

ALTER TABLE IF EXISTS tickets ADD CONSTRAINT fk_ticket_types FOREIGN KEY(ticket_type_id) REFERENCES ticket_types("id");
