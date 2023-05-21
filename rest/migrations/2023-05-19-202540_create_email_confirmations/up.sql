-- Your SQL goes here

CREATE TABLE IF NOT EXISTS email_confirmations (
    "id" INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    "key" VARCHAR(600) NOT NULL,
    "user_id" INTEGER NOT NULL,

    CONSTRAINT fk_email_confirmations_users FOREIGN KEY("user_id") REFERENCES users("id")
)
