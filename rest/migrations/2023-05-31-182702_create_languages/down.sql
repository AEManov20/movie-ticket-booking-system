-- This file should undo anything in `up.sql`

ALTER TABLE IF EXISTS theatre_screenings DROP CONSTRAINT IF EXISTS subtitles_language_id_fkey;
ALTER TABLE IF EXISTS theatre_screenings DROP CONSTRAINT IF EXISTS audio_language_id_fkey;
DROP TABLE IF EXISTS languages;
