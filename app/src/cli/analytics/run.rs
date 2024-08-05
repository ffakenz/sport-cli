use crate::{
    db::Db,
    scrapper::{Query, Scrapper},
};

use anyhow::Result;
use engine::{
    engine::{Dimension, Engine, MetricKind, Query as EngineQuery, QueryResponse, Sort},
    repo::{in_memo::InMemoRepository, model::PlayerDetails},
};
use sport_radar::client::SportRadarClient;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn run(query: Query) -> Result<()> {
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
