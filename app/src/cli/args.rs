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
    /// Perform analytics queries on sports data.
    Analytics(AnalyticsArgs),
    /// Perform queries on sport competitions data.
    Competitions(CompetitionsArgs),
    /// Perform queries on sport seasons data, given competition.
    Seasons(SeasonsArgs),
    /// Perform queries on sport teams data, given competition and season.
    Teams(TeamsArgs),
    /// Perform queries on sport players data, given competition and season.
    Players(PlayersArgs),
}
