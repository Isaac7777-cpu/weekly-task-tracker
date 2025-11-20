use clap::{Parser, Subcommand};

/// Weekly commitment tracker
#[derive(Parser)]
#[command(
    name = "Weekly Progress Tracker",
    about = "Track weekly commitments and hours",
    version = "0.0.1",
    author = "Isaac Leong"
)]
#[derive(Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add { name: String, weekly_hours: f64 },
    Archive { id: i64 },
    Reactivate { id: i64 },
    Log { id: i64, hours: f32 },
    List,
    History { id: i64 },
}
