-- Your SQL goes here

CREATE TABLE IF NOT EXISTS theatre_movies (
    "id" INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    movie_id INTEGER NOT NULL REFERENCES movies("id"),
    hall_id INTEGER NOT NULL REFERENCES halls("id"),
    subtitles_language VARCHAR(50),
    audio_language VARCHAR(50) NOT NULL,
    starting_time TIMESTAMP NOT NULL,
    -- 0 - not yet started
    -- 1 - adverts are running
    -- 2 - movie is running
    -- 3 - movie is finished
    "status" INTEGER NOT NULL DEFAULT 0
);
