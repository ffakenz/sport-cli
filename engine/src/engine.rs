use crate::repo::{Gender, Metric as RepoMetric, Player, PlayerStats, PlayerStatsRepo};
use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Engine;

impl Engine {
    pub fn execute(&self, repo: PlayerStatsRepo, query: Query) -> Vec<QueryResponse<Player>> {
        let data: &Vec<PlayerStats> = repo.all();

        // Filter, transform the data
        let mut player_scores: Vec<(Player, Metric, u32)> = data
            .iter()
            .filter(|player_stats| {
                player_stats.competition.name == query.event
                    && player_stats.competition.location == query.location
                    && player_stats.competition.gender == query.gender
                    && player_stats.competition.season_start == query.season_start
                    && player_stats.competition.season_end == query.season_end
            })
            .map(|stats| {
                let metric_value = match query.metric {
                    Metric::GoalsScored => stats
                        .metrics
                        .iter()
                        .find_map(|m| match m {
                            RepoMetric::GoalsScored { value } => Some(*value),
                            _ => None,
                        })
                        .unwrap_or(0),
                    Metric::Assists => stats
                        .metrics
                        .iter()
                        .find_map(|m| match m {
                            RepoMetric::Assists { value } => Some(*value),
                            _ => None,
                        })
                        .unwrap_or(0),
                };

                (stats.player.clone(), query.metric.clone(), metric_value)
            })
            .collect();

        // Sort the players based on the metric value
        player_scores.sort_by(|a, b| match query.sort {
            Sort::Asc => a.2.cmp(&b.2),
            Sort::Desc => b.2.cmp(&a.2),
        });

        // Limit the results
        let limited: Vec<QueryResponse<Player>> = player_scores
            .into_iter()
            .take(query.limit as usize)
            .map(|(player, metric, value)| QueryResponse {
                dimension: player,
                metric,
                value,
            })
            .collect();

        limited
    }
}

// --------------------------------------------------
// Model for "query" engine
// --------------------------------------------------

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Dimension {
    #[default]
    Player,
    // TODO! support other dimensions
    Team,
}
unsafe impl Send for Dimension {}
unsafe impl Sync for Dimension {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Sort {
    #[default]
    Asc,
    Desc,
}
unsafe impl Send for Sort {}
unsafe impl Sync for Sort {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Metric {
    #[default]
    #[serde(rename = "goals_scored")]
    GoalsScored,
    Assists,
    // TODO! support other metrics
}
unsafe impl Send for Metric {}
unsafe impl Sync for Metric {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Query {
    pub event: String,
    pub location: String,
    pub season_start: NaiveDate,
    pub season_end: NaiveDate,
    pub gender: Gender,
    pub dimension: Dimension,
    // TODO! support multiple metrics
    pub metric: Metric,
    // TODO! support sorting by multiple fields
    pub sort: Sort,
    pub limit: u8,
}
unsafe impl Send for Query {}
unsafe impl Sync for Query {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct QueryResponse<T> {
    pub dimension: T,
    pub metric: Metric,
    pub value: u32,
}
unsafe impl<T> Send for QueryResponse<T> {}
unsafe impl<T> Sync for QueryResponse<T> {}
