-- This file should undo anything in `up.sql`

ALTER TABLE IF EXISTS tickets DROP CONSTRAINT IF EXISTS tickets_ticket_types_fkey;
DROP TABLE IF EXISTS ticket_types;
