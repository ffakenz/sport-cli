use anyhow::Result;
use chrono::NaiveDate;
use engine::{
    engine::{Dimension, Engine, Metric, Query as EngineQuery, QueryResponse, Sort},
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
    let repo: PlayerStatsRepo = scrapper.execute(sport_data_source, &query).await?;
    dbg!("nbr of scrapps: {:?}", repo.all().len());

    // Execute the engine query
    let engine = Engine;
    let query = EngineQuery {
        event: query.event,
        location: query.location,
        gender: query.gender,
        season_start: query.season_start,
        season_end: query.season_end,
        dimension: Dimension::Player,
        metric: Metric::GoalsScored,
        sort: Sort::Desc,
        limit: 2,
    };

    let results: Vec<QueryResponse<Player>> = engine.execute(repo, query);

    // Print the results (for demonstration purposes)
    for result in results {
        dbg!("{:?}", result);
    }

    Ok(())
}
