-- Your SQL goes here

CREATE TABLE IF NOT EXISTS languages (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "code" CHAR(2) UNIQUE NOT NULL,
    "name" VARCHAR(150) UNIQUE NOT NULL
);

ALTER TABLE IF EXISTS theatre_screenings ADD CONSTRAINT subtitles_language_fkey FOREIGN KEY(subtitles_language) REFERENCES languages("id");
ALTER TABLE IF EXISTS theatre_screenings ADD CONSTRAINT audio_language_fkey FOREIGN KEY(audio_language) REFERENCES languages("id");
