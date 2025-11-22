mod cli;
mod db;
mod model;
mod util;

use clap::Parser;
use cli::Cli;

use crate::{
    cli::Commands,
    db::{
        add_commitment, archive_commiment, current_week_progress_by_id, get_commitment,
        list_commitments_with_week_progress, log_record, log_record_id, open_db,
        reactivate_commiment,
    },
    util::{color_for_pct, render_progress_bar},
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
            let mut commitments = list_commitments_with_week_progress(&pool).await?;
            if commitments.is_empty() {
                println!("No active commiments.");
            } else {
                commitments.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
                println!("Active commiments:\n");
                for commitment in commitments {
                    let current = commitment.week_total.unwrap_or(0.0);
                    let status_note = if commitment.week_total.is_none() {
                        " (Haven't started this week...)"
                    } else {
                        ""
                    };

                    let pct = if commitment.weekly_target_hours > 0.0 {
                        (current / commitment.weekly_target_hours * 100.0).clamp(0.0, 999.9)
                    } else {
                        0.0
                    };

                    let message = format!(
                        "{cur:.1}/{target:.1} h ({pct:.1}%){note}",
                        cur = current,
                        target = commitment.weekly_target_hours,
                        pct = pct,
                        note = status_note
                    );

                    let bar = render_progress_bar(
                        current,
                        commitment.weekly_target_hours,
                        message.len() + 5, // +2 for the "  " before the message
                    );

                    let color = color_for_pct(pct);
                    const RESET: &str = "\x1b[0m";

                    let colored_message = format!(
                        "{cur:.1}/{target:.1} h {color}({pct:.1}%){RESET}\x1b[31m{note}\x1b[0m",
                        cur = current,
                        target = commitment.weekly_target_hours,
                        pct = pct,
                        note = status_note
                    );

                    println!(
                        "[#{id}] {name}\n {bar}  {message}",
                        id = commitment.id,
                        name = commitment.name,
                        bar = bar,
                        message = colored_message
                    );
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
                assert!(ct.id == id);

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
