# Weekly Commitment Tracker

A lightweight CLI tool written in **Rust** for tracking weekly commitments, logging progress, and maintaining a history of personal productivity.  
The project uses **SQLite** and **SQLx** for async database access, along with **Clap** for command-line parsing.

![GitHub commit activity](https://img.shields.io/github/commit-activity/m/Isaac7777-cpu/weekly-task-tracker?style=for-the-badge&labelColor=%231d4971&color=%233b82c1)
![GitHub contributors](https://img.shields.io/github/contributors/Isaac7777-cpu/weekly-task-tracker?style=for-the-badge&labelColor=%23241908&color=%23ad5700)
![GitHub last commit](https://img.shields.io/github/last-commit/Isaac7777-cpu/weekly-task-tracker?style=for-the-badge&labelColor=%232f332f&color=%235e7f6a)
![GitHub License](https://img.shields.io/github/license/Isaac7777-cpu/weekly-task-tracker?style=for-the-badge&labelColor=%232f2b36&color=%235c6bc0)
![GitHub repo size](https://img.shields.io/github/repo-size/Isaac7777-cpu/weekly-task-tracker?style=for-the-badge&labelColor=%23004c4c&color=%23b2d8d8)

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
│
├ # CLI parts with application backends (database parts)
├── db.rs # Database setup + SQLx queries
├── main.rs # Entry point + routing commands
├── model.rs # Structs representing DB rows
├── cli.rs # Command-line argument definitions (Clap)
│
├ # TUI part, vaguely follow MVC architecture but also uses the above database as the 'backend'
├── app.rs # TUI App state (Model)
├── ui.rs # TUI UI Functions (View)
└── tui.rs # TUI main loop + Event Handling (Controller)

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
- [ ] Add Testing for the Functionalities
- [ ] Add TUI mode using `ratatui` (Mostly Done)
- [ ] Integrate with `neovim` / `vim`
- [ ] Export data to CSV
- [x] Graph weekly progress (e.g., via `ratatui`)
- [ ] Allow for having "Off-Weeks"
- [ ] Sync across devices

---

## Contribution Guide

Thanks for your interest in contributing! All contributions are welcome — whether it’s fixing a bug, improving documentation, or adding new features.

1. Make sure you have **Rust (stable)** installed.
2. Clone the repository:
3. You can directly build the project and run it with
   ```sh
   cargo run
   ```
4. Use SQLx CLI for migrations:
   ```sh
   cargo install sqlx-cli
   sqlx run migrations
   ```
   Note that currently the project relies on a relative path to find the database (terrible idea, happy to take PR), so you have to run `cargo run` in the top level directory, and you should / can use `sqlx create database` to create the database.

This Project uses Rust's standard formatting (`rustfmt`), please use this formatting for the project.

---

## Tech Stack

<p align="center">
  <img src="https://github-readme-tech-stack.vercel.app/api/cards?title=Weekly+Commitment+Tracker+Tech+Stack&align=center&titleAlign=center&fontSize=20&lineHeight=10&lineCount=2&theme=ayu&width=520&line1=rust,rust,DEA584;tokio,tokio,007ACC;clap,clap,FF7F2A;sqlx,sqlx,3A3A3A;&line2=sqlite,sqlite,003B57;ratatui,ratatui,FFC107;crossterm,crossterm,4A90E2;" alt="Tech Stack" />
</p>
