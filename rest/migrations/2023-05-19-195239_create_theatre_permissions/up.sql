-- Your SQL goes here

CREATE TABLE IF NOT EXISTS theatre_permissions (
    "id" INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    "user_id" INTEGER NOT NULL,
    theatre_id INTEGER NOT NULL,
    can_manage_users BOOLEAN NOT NULL DEFAULT FALSE,
    can_manage_movies BOOLEAN NOT NULL DEFAULT FALSE,
    can_check_tickets BOOLEAN NOT NULL DEFAULT FALSE,
    can_manage_tickets BOOLEAN NOT NULL DEFAULT FALSE,
    is_theatre_owner BOOLEAN NOT NULL DEFAULT FALSE,

    CONSTRAINT fk_theatre_permissions_users FOREIGN KEY("user_id") REFERENCES users("id"),
    CONSTRAINT fk_theatre_permissions_theatres FOREIGN KEY(theatre_id) REFERENCES theatres("id")
);
