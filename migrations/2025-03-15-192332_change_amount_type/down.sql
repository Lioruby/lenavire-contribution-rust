-- This file should undo anything in `up.sql`
ALTER TABLE payments ALTER COLUMN amount TYPE FLOAT8;
ALTER TABLE expenses ALTER COLUMN amount TYPE FLOAT8;