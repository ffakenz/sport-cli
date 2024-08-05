use chrono::NaiveDate;
use clap::Args;

use crate::cli::utils::{GenderArg, SportArg};

#[derive(Debug, Args)]
pub struct PlayersArgs {
    #[arg(long, value_enum, short = 'S')]
    /// Sport to analyze (football)
    pub sport: SportArg,
    #[arg(long, short = 'E')]
    /// The event name, such as "Premier's League"
    pub event: String,
    #[arg(long, short = 'L')]
    /// The location of the event, such as England, International, etc...
    pub location: String,
    #[arg(long, value_enum, short = 'G')]
    /// Analyze data by gender (male, female)
    pub gender: GenderArg,
    #[arg(long)]
    /// The start date of the season in YYYY-MM-DD
    pub season_start: NaiveDate,
    #[arg(long)]
    /// The end date of the season in YYYY-MM-DD
    pub season_end: NaiveDate,
}
