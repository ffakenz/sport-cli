use anyhow::Result;
use chrono::NaiveDate;
use engine::{
    engine::{Dimension, Engine, Metric, Query as EngineQuery, Sort},
    repo::{Gender, Player, PlayerStatsRepo},
};
use sport_radar::client::SportRadarClient;
use std::sync::Arc;

mod scrapper;
use scrapper::{Query, Scrapper};

#[tokio::main]
async fn main() -> Result<()> {
    // Set up the hardcoded query
    let query = Query {
        event: "Premier League".to_string(),
        location: "England".to_string(),
        gender: Gender::Male,
        season_start: NaiveDate::from_ymd_opt(2023, 8, 11).unwrap(),
        season_end: NaiveDate::from_ymd_opt(2024, 5, 19).unwrap(),
    };

    // Initialize the SportRadarClient
    let sport_data_source = Arc::new(SportRadarClient::from_env());

    // Create a scrapper instance
    let scrapper = Scrapper;

    // Execute the scrapper with the query
    let repo: PlayerStatsRepo = scrapper.execute(sport_data_source, query).await?;

    // Print the player stats (for demonstration purposes)
    for player_stats in repo.all() {
        dbg!("{:?}", player_stats);
    }

    // Execute the engine query
    let engine = Engine;
    let query = EngineQuery {
        event: "Premier League".to_string(),
        location: "England".to_string(),
        season_start: NaiveDate::from_ymd_opt(2023, 8, 11).unwrap(),
        season_end: NaiveDate::from_ymd_opt(2024, 5, 19).unwrap(),
        gender: Gender::Male,
        dimension: Dimension::Player,
        metric: Metric::GoalsScored,
        sort: Sort::Desc,
        limit: 2,
    };

    let results: Vec<Player> = engine.execute(repo, query);

    // Print the results (for demonstration purposes)
    for result in results {
        dbg!("{:?}", result);
    }

    Ok(())
}
