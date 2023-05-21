-- Your SQL goes here

CREATE TABLE IF NOT EXISTS theatre_movies (
    "id" INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    movie_id INTEGER NOT NULL,
    hall_id INTEGER NOT NULL,
    subtitles_language VARCHAR(50),
    audio_language VARCHAR(50) NOT NULL,
    starting_time TIMESTAMP NOT NULL,

    CONSTRAINT fk_theatre_movies_halls FOREIGN KEY(hall_id) REFERENCES halls("id"),
    CONSTRAINT fk_theatre_movies_movies FOREIGN KEY(movie_id) REFERENCES movies("id")
);
