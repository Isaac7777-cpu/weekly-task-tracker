ALTER TABLE commitments
ADD COLUMN created_at TEXT;

UPDATE commitments
SET created_at = CURRENT_TIMESTAMP
WHERE created_at IS NULL;
