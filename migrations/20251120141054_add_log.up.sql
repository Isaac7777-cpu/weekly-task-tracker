CREATE TABLE progress_logs (
  id INTEGER PRIMARY Key AUTOINCREMENT,
  commiment_id INTEGER NOT NULL,
  hours REAL NOT NULL,
  logged_at TEXT NOT NULL,
  FOREIGN KEY (commiment_id) REFERENCES commiments(id)
);
