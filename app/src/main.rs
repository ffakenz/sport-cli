mod db;
use db::Db;

use anyhow::Result;
use chrono::NaiveDate;
use engine::{
    engine::{Dimension, Engine, MetricKind, Query as EngineQuery, QueryResponse, Sort},
    repo::{
        in_memo::InMemoRepository,
        model::{Gender, PlayerDetails},
    },
};
use sport_radar::client::SportRadarClient;
use std::sync::Arc;
use tokio::sync::Mutex;

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

    let db = Arc::new(Mutex::new(Db::new()));
    let sport_data_source = Arc::new(SportRadarClient::from_env());

    Scrapper
        .execute(sport_data_source, &query, Arc::clone(&db))
        .await?;

    let db = db.lock().await;
    dbg!(db.competitions.all().len());
    dbg!(db.teams.all().len());
    dbg!(db.players.all().len());
    dbg!(db.players_stats.all().len());

    let query = EngineQuery {
        event: query.event,
        location: query.location,
        gender: query.gender,
        season_start: query.season_start,
        season_end: query.season_end,
        dimension: Dimension::Player,
        metric: MetricKind::GoalsScored,
        sort: Sort::Desc,
        limit: 2,
    };

    let results: Vec<QueryResponse<PlayerDetails>> = Engine.execute(
        &db.players,
        &db.teams,
        &db.players_stats,
        &db.competitions,
        &query,
    );

    for result in results {
        dbg!(result);
    }

    Ok(())
}
