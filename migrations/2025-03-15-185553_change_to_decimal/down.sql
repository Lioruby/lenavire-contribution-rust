-- This file should undo anything in `up.sql`
ALTER TABLE expenses ALTER COLUMN amount TYPE FLOAT;
ALTER TABLE payments ALTER COLUMN amount TYPE FLOAT;