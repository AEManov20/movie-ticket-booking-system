-- Your SQL goes here

CREATE TABLE movie_reviews (
    "id" INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    author_user_id INTEGER NOT NULL REFERENCES users("id"),
    movie_id INTEGER NOT NULL REFERENCES movies("id"),
    "content" VARCHAR(2500),
    rating FLOAT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    votes INTEGER NOT NULL DEFAULT 0
);