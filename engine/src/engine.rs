use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};

use crate::repo::{
    in_memo::InMemoRepository,
    model::{Competition, Gender, Metric, Player, PlayerDetails, PlayerStats, Team},
};

#[derive(Debug, Clone)]
pub struct Engine;

impl Engine {
    pub fn execute<'a>(
        &self,
        players: &'a impl InMemoRepository<Player>,
        teams: &'a impl InMemoRepository<Team>,
        player_stats: &'a impl InMemoRepository<PlayerStats>,
        competitions: &'a impl InMemoRepository<Competition>,
        query: &'a Query,
    ) -> Vec<QueryResponse<PlayerDetails<'a>>> {
        let default_metric = &Metric::GoalsScored { value: 0 };

        let player_scores: Vec<(&PlayerStats, &Metric)> = player_stats
            .filter_iter(move |player_stats| {
                competitions
                    .all()
                    .get(&player_stats.competition_id)
                    .map_or(false, |competition| {
                        competition.name == query.event
                            && competition.location == query.location
                            && competition.gender == query.gender
                            && competition.season_start == query.season_start
                            && competition.season_end == query.season_end
                    })
            })
            .map(move |(_, player_stats)| {
                let metric = match query.metric {
                    MetricKind::GoalsScored => player_stats
                        .metrics
                        .iter()
                        .find(|m| matches!(m, Metric::GoalsScored { value: _ }))
                        .unwrap_or(default_metric),
                    MetricKind::Assists => player_stats
                        .metrics
                        .iter()
                        .find(|m| matches!(m, Metric::Assists { value: _ }))
                        .unwrap_or(default_metric),
                };

                (player_stats, metric)
            })
            .collect();

        let mut sorted_scores = player_scores;
        sorted_scores.sort_by(|a, b| match query.sort {
            Sort::Asc => a.1.value().cmp(&b.1.value()),
            Sort::Desc => b.1.value().cmp(&a.1.value()),
        });

        sorted_scores
            .into_iter()
            .take(query.limit as usize)
            .map(|(player_stats, metric)| {
                let team = teams.find(&player_stats.team_id).unwrap();
                let player = players.find(&player_stats.player_id).unwrap();
                let competition = competitions.find(&player_stats.competition_id).unwrap();
                let player_details = PlayerDetails {
                    player_id: &player.id,
                    player_name: &player.name,
                    team_id: &team.id,
                    team_name: &team.name,
                    competition_id: &competition.id,
                    competition_name: &competition.name,
                };
                QueryResponse {
                    dimension: player_details,
                    metric: query.metric.clone(),
                    value: metric.value(),
                }
            })
            .collect()
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
pub enum MetricKind {
    #[default]
    #[serde(rename = "goals_scored")]
    GoalsScored,
    Assists,
    // TODO! support other metrics
}
unsafe impl Send for MetricKind {}
unsafe impl Sync for MetricKind {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Query {
    pub event: String,
    pub location: String,
    pub season_start: NaiveDate,
    pub season_end: NaiveDate,
    pub gender: Gender,
    pub dimension: Dimension,
    // TODO! support multiple metrics
    pub metric: MetricKind,
    // TODO! support sorting by multiple fields
    pub sort: Sort,
    pub limit: u8,
}
unsafe impl Send for Query {}
unsafe impl Sync for Query {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct QueryResponse<T> {
    pub dimension: T,
    // TODO! use Arc
    pub metric: MetricKind,
    pub value: u32,
}
unsafe impl<T> Send for QueryResponse<T> {}
unsafe impl<T> Sync for QueryResponse<T> {}
