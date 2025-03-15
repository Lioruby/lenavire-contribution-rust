-- Your SQL goes here

-- Modification de la colonne amount dans la table payments
ALTER TABLE payments
ALTER COLUMN amount TYPE NUMERIC(20,2);

-- Modification de la colonne amount dans la table expenses
ALTER TABLE expenses
ALTER COLUMN amount TYPE NUMERIC(20,2);
