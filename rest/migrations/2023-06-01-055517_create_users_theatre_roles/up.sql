-- Your SQL goes here

CREATE TABLE IF NOT EXISTS users_theatre_roles (
    "user_id" UUID NOT NULL REFERENCES users("id"),
    "role_id" UUID NOT NULL REFERENCES theatre_roles("id"),
    "theatre_id" UUID NOT NULL REFERENCES theatres("id"),

    PRIMARY KEY("user_id", "role_id", "theatre_id")
);
