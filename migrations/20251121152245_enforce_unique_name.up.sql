-- 1. Create new table with UNIQUE constraint
CREATE TABLE commitments_new (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL UNIQUE,
  weekly_target_hours REAL NOT NULL,
  active INTEGER NOT NULL,
  created_at TEXT,
  start_week_monday TEXT
);

-- 2. Copy old data
INSERT INTO
  commitments_new (
    id,
    name,
    weekly_target_hours,
    active,
    created_at,
    start_week_monday
  )
SELECT
  id,
  name,
  weekly_target_hours,
  active,
  created_at,
  start_week_monday
FROM
  commitments;

-- 3. Drop old table
DROP TABLE commitments;

-- 4. Rename new table
ALTER TABLE commitments_new
RENAME TO commitments;
