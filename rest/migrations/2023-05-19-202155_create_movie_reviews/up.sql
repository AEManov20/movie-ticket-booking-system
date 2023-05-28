-- Your SQL goes here

CREATE TABLE movie_reviews (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    author_user_id UUID NOT NULL REFERENCES users("id"),
    movie_id UUID NOT NULL REFERENCES movies("id"),
    "content" VARCHAR(2500),
    rating FLOAT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    votes INTEGER NOT NULL DEFAULT 0
);