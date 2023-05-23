-- Your SQL goes here

CREATE TABLE IF NOT EXISTS external_credentials (
    "id" INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    "provider" VARCHAR(50) NOT NULL,
    external_id VARCHAR(150) NOT NULL,
    "user_id" INTEGER NOT NULL REFERENCES users("id")
)
