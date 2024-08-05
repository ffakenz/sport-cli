use crate::cli::utils::{DimensionArg, GenderArg, MetricArg, SortArg, SportArg};

use chrono::NaiveDate;
use clap::Args;

#[derive(Debug, Args)]
pub struct AnalyticsArgs {
    #[arg(long)]
    pub sport: SportArg,
    #[arg(long)]
    pub event: String,
    #[arg(long)]
    pub location: String,
    #[arg(long)]
    pub season_start: NaiveDate,
    #[arg(long)]
    pub season_end: NaiveDate,
    #[arg(long)]
    pub dimension: DimensionArg,
    #[arg(long)]
    pub metric: MetricArg,
    #[arg(long)]
    pub gender: GenderArg,
    #[arg(long)]
    pub sort: SortArg,
    #[arg(long)]
    pub limit: u32,
    #[arg(long)]
    pub timeout: Option<u32>,
}
