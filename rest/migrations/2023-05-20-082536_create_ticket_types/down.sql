-- This file should undo anything in `up.sql`

ALTER TABLE IF EXISTS tickets DROP CONSTRAINT IF EXISTS fk_ticket_types;
ALTER TABLE IF EXISTS ticket_types DROP CONSTRAINT IF EXISTS fk_ticket_types_theatres;
DROP TABLE IF EXISTS ticket_types;
