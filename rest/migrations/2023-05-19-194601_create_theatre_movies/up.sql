-- Your SQL goes here

CREATE TABLE IF NOT EXISTS theatre_movies (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    movie_id UUID NOT NULL REFERENCES movies("id"),
    theatre_id UUID NOT NULL REFERENCES theatres("id"),
    hall_id UUID NOT NULL REFERENCES halls("id"),
    subtitles_language VARCHAR(50),
    audio_language VARCHAR(50) NOT NULL,
    starting_time TIMESTAMP NOT NULL,
    -- 0 - not yet started
    -- 1 - adverts are running
    -- 2 - movie is running
    -- 3 - movie is finished
    "status" INTEGER NOT NULL DEFAULT 0
);
