-- Your SQL goes here

CREATE TABLE IF NOT EXISTS theatre_permissions (
    "user_id" UUID NOT NULL REFERENCES users("id"),
    theatre_id UUID NOT NULL REFERENCES theatres("id"),
    can_manage_users BOOLEAN NOT NULL DEFAULT FALSE,
    can_manage_movies BOOLEAN NOT NULL DEFAULT FALSE,
    can_check_tickets BOOLEAN NOT NULL DEFAULT FALSE,
    can_manage_tickets BOOLEAN NOT NULL DEFAULT FALSE,
    is_theatre_owner BOOLEAN NOT NULL DEFAULT FALSE,

    PRIMARY KEY("user_id", theatre_id)
);
