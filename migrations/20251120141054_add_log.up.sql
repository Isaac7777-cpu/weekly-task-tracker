CREATE TABLE progress_logs (
  id INTEGER PRIMARY Key AUTOINCREMENT,
  commitment_id INTEGER NOT NULL,
  hours REAL NOT NULL,
  logged_at TEXT NOT NULL,
  FOREIGN KEY (commitment_id) REFERENCES commitments(id)
);
