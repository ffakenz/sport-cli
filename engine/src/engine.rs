use crate::repo::{Gender, Metric as RepoMetric, Player, PlayerStats, PlayerStatsRepo};
use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Engine;

impl Engine {
    pub fn execute(&self, repo: PlayerStatsRepo, query: Query) -> Vec<Player> {
        let data: &Vec<PlayerStats> = repo.all();

        // Filter, transform the data
        let mut player_scores: Vec<(Player, u32)> = data
            .iter()
            .filter(|player_stats| {
                player_stats.competition.name == query.event
                    && player_stats.competition.location == query.location
                    && player_stats.competition.gender == query.gender
                    // FIXME! relax date comparison
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

                (stats.player.clone(), metric_value)
            })
            .collect();

        // Sort the players based on the metric value
        player_scores.sort_by(|a, b| match query.sort {
            Sort::Asc => a.1.cmp(&b.1),
            Sort::Desc => b.1.cmp(&a.1),
        });

        // Limit the results
        let limited: Vec<Player> = player_scores
            .into_iter()
            .take(query.limit as usize)
            .map(|(player, _)| player)
            .collect();

        limited
    }
}

// --------------------------------------------------
// Model for "query" engine
// --------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Dimension {
    Player,
    // TODO! support other dimensions
    Team,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Sort {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Metric {
    GoalsScored,
    Assists,
    // TODO! support other metrics
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
