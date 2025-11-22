mod cli;
mod db;
mod model;

use clap::Parser;
use cli::Cli;
use db::open_db;

use crate::{
    cli::Commands,
    db::{
        add_commitment, archive_commiment, current_week_progress_by_id, get_commitment,
        list_commitments, log_record, log_record_id, reactivate_commiment,
    },
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

        Commands::Reactivate { id } => {
            let num_reactivated = reactivate_commiment(&pool, id).await?;
            if num_reactivated > 0 {
                println!("Marked commitment #{id} as active. (Affected {num_reactivated} rows)");
            } else {
                eprintln!("No inactive commiment with id {id}.");
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

        Commands::LogID {
            id: commitment_id,
            hours,
        } => {
            let id = log_record_id(&pool, commitment_id, hours).await?;

            println!("Logged record #{id} for commitment #{commitment_id} for {hours} hours.");
        }

        Commands::Log { name, hours } => {
            let id = log_record(&pool, name.as_str(), hours).await?;

            println!("Logged record #{id} for commitment '{name}' for {hours} hours.");
        }

        Commands::TrackID { id } => {
            let week_total = current_week_progress_by_id(&pool, id).await?;
            let commitment = get_commitment(&pool, id).await?;

            if let Some(wk) = week_total
                && let Some(ct) = commitment
            {
                if !ct.active {
                    eprintln!("The activity is currently not active.");
                }

                println!(
                    "Current week progress for task '{}' is {}/{}",
                    ct.name, wk, ct.weekly_target_hours
                );
            } else if commitment.is_none() {
                eprintln!("Cannot find commitment #{id}.");
            } else if week_total.is_none() {
                let ct = commitment.unwrap();
                eprintln!("You have not started on task '{}' this week.", ct.name);
            }
        }

        x => {
            println!("{:?} not implemented yet.", x);
        }
    }

    Ok(())
}
