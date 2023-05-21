-- Your SQL goes here

CREATE TABLE movie_reviews (
    "id" INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    author_user_id INTEGER NOT NULL,
    movie_id INTEGER NOT NULL,
    "content" VARCHAR(2500),
    rating FLOAT NOT NULL,

    CONSTRAINT fk_movie_reviews_users FOREIGN KEY(author_user_id) REFERENCES users("id"),
    CONSTRAINT fk_movie_reviews_movies FOREIGN KEY(movie_id) REFERENCES movies("id")
);