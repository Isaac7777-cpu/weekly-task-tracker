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
    Add { name: String, weekly_hours: f32 },
    Remove { id: u32 },
    Log { id: u32, hours: f32 },
    List,
    History { id: u32 },
}

fn main() {
    let cli = Cli::parse();
    println!("{:#?}", cli);
}
