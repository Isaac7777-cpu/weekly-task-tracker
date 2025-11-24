# Weekly Commitment Tracker

A lightweight CLI tool written in **Rust** for tracking weekly commitments, logging progress, and maintaining a history of personal productivity.  
The project uses **SQLite** and **SQLx** for async database access, along with **Clap** for command-line parsing.

---

## ✨ Features

- **Add commitments** with weekly target hours  
- **Log progress** (e.g., daily or weekly hours done)  
- **List all active commitments**  
- **Archive / Reactivate** commitments without deleting history  
- **View current week's progress** (per commitment and total)  
- **SQLite-backed**, async, and easy to migrate

---

## 📦 Project Structure

```
src/
├── cli.rs # Command-line argument definitions (Clap)
├── db.rs # Database setup + SQLx queries
├── main.rs # Entry point + routing commands
└── models.rs # (If used) Structs representing DB rows
migrations/
└── <timestamp>.sql # SQLx migration
```

---

## ✨ Features / TODO

- [x] Add commitments with weekly target hours  
- [x] Log progress (daily/weekly hours)  
- [x] List all active commitments  
- [x] Archive commitments  
- [x] Reactivate archived commitments  
- [x] View current week progress (per commitment + total)  
- [ ] Add database indexes for faster queries  
- [ ] Add TUI mode using `ratatui`  
- [ ] Export data to CSV  
- [ ] Graph weekly progress (e.g., via `plotters`)  
- [ ] Sync across devices  

