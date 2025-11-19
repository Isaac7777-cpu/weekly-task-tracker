mod cli;
mod db;

use clap::Parser;
use cli::Cli;
use db::open_db;

use crate::{
    cli::Commands,
    db::{add_commitment, archive_commiment, list_commitments},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let pool = open_db().await;

    match cli.command {
        Commands::Add { name, weekly_hours } => {
            let id = add_commitment(&pool, &name, weekly_hours).await?;

            println!(
                "Added commitment #{id}: '{}' ({} hours/week)",
                name, weekly_hours
            );
        }

        Commands::Archive { id } => {
            let num_archived = archive_commiment(&pool, id).await?;
            if num_archived > 0 {
                println!("Marked commitment #{id} as inactive. (Affected {num_archived} rows)");
            } else {
                eprintln!("No active commitment with id {id}.")
            }
        }

        Commands::List => {
            let commitments = list_commitments(&pool).await;
            if commitments.is_empty() {
                println!("No active commiments.");
            } else {
                println!("Active commiments:\n");
                for commitment in commitments {
                    println!(
                        "[#{id}] {name}\n Target: {hours:.1} hours/week\n",
                        id = commitment.id,
                        name = commitment.name,
                        hours = commitment.weekly_target_hours
                    )
                }
            }
        }

        x => {
            println!("{:?} not implemented yet.", x);
        }
    }

    Ok(())
}
