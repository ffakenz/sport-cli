use crate::cli::utils::{DimensionArg, GenderArg, MetricArg, SortArg, SportArg};

use chrono::NaiveDate;
use clap::Args;

#[derive(Debug, Args)]
pub struct AnalyticsArgs {
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
    #[arg(long, value_enum, short = 'd')]
    /// Entity to analyze (player, team)
    pub dimension: DimensionArg,
    #[arg(long, value_enum, short = 'm')]
    /// Metric to analyze (score, assist)
    pub metric: MetricArg,
    #[arg(long, value_enum, short = 's')]
    /// Analyze data in order (asc, desc)
    pub sort: SortArg,
    #[arg(long, value_enum, short = 'l')]
    /// The maximum number of results to return
    pub limit: u32,
    #[arg(long, value_enum, short = 't')]
    /// (optional) Timeout for the request in millis
    pub timeout: Option<u32>,
}
