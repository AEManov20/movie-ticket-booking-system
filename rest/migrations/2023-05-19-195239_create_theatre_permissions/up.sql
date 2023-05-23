-- Your SQL goes here

CREATE TABLE IF NOT EXISTS theatre_permissions (
    "id" INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    "user_id" INTEGER NOT NULL REFERENCES users("id"),
    theatre_id INTEGER NOT NULL REFERENCES theatres("id"),
    can_manage_users BOOLEAN NOT NULL DEFAULT FALSE,
    can_manage_movies BOOLEAN NOT NULL DEFAULT FALSE,
    can_check_tickets BOOLEAN NOT NULL DEFAULT FALSE,
    can_manage_tickets BOOLEAN NOT NULL DEFAULT FALSE,
    is_theatre_owner BOOLEAN NOT NULL DEFAULT FALSE
);
