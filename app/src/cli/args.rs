use clap::{Parser, Subcommand};

use super::{
    analytics::args::AnalyticsArgs, competitions::args::CompetitionsArgs,
    players::args::PlayersArgs, seasons::args::SeasonsArgs, teams::args::TeamsArgs,
};

#[derive(Debug, Parser)]
#[command(
    name = "sport-cli",
    author = "ffakenz",
    about = "CLI tool for sports analytics",
    version = "1.0"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Analytics(AnalyticsArgs),
    Competitions(CompetitionsArgs),
    Seasons(SeasonsArgs),
    Teams(TeamsArgs),
    Players(PlayersArgs),
}
