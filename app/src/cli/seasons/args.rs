use chrono::NaiveDate;
use clap::Args;

#[derive(Debug, Args)]
pub struct SeasonsArgs {
    #[arg(long)]
    // TODO! enum
    sport: String,
    #[arg(long)]
    event: String,
    #[arg(long)]
    location: String,
    #[arg(long)]
    season_start: NaiveDate,
    #[arg(long)]
    season_end: NaiveDate,
}
