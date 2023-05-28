-- Your SQL goes here

CREATE TABLE IF NOT EXISTS external_credentials (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "provider" VARCHAR(50) NOT NULL,
    external_id VARCHAR(150) NOT NULL,
    "user_id" UUID NOT NULL REFERENCES users("id")
)
