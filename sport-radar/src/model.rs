use serde_derive::{Deserialize, Serialize};

// --------------------------------------------------
// Model for the "competitions" endpoint
// --------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitionCategory {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]

pub enum CompetitionGender {
    Men,
    Women,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Competition {
    pub id: String,
    pub name: String,
    pub gender: Option<CompetitionGender>,
    pub category: CompetitionCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitionsResponse {
    pub generated_at: String,
    pub competitions: Vec<Competition>,
}

// --------------------------------------------------
// Model for the "competition_seasons" endpoint
// --------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Season {
    pub id: String,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub year: String,
    pub competition_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonsResponse {
    pub generated_at: String,
    pub seasons: Vec<Season>,
}

// --------------------------------------------------
// Model for the "season_competitors" endpoint
// --------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Competitor {
    pub id: String,
    pub name: String,
    pub short_name: String,
    pub abbreviation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitorsResponse {
    pub generated_at: String,
    pub season_competitors: Vec<Competitor>,
}

// --------------------------------------------------
// Model for the "seasonal_competitor_statistics" endpoint
// --------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sport {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonDetails {
    pub id: String,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub year: String,
    pub competition_id: String,
    pub sport: Sport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStatistics {
    pub assists: u32,
    pub cards_given: Option<u32>,
    pub goals_by_head: u32,
    pub goals_by_penalty: u32,
    pub goals_conceded: u32,
    pub goals_scored: u32,
    pub matches_played: u32,
    pub offsides: Option<u32>,
    pub own_goals: u32,
    pub penalties_missed: u32,
    pub red_cards: u32,
    pub shots_off_target: Option<u32>,
    pub substituted_in: u32,
    pub substituted_out: u32,
    pub yellow_cards: u32,
    pub yellow_red_cards: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub statistics: PlayerStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]

pub enum CompetitorGender {
    Male,
    Female,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitorDetails {
    pub id: String,
    pub name: String,
    pub country: String,
    pub country_code: String,
    pub abbreviation: String,
    pub gender: CompetitorGender,
    pub statistics: CompetitorStatistics,
    pub players: Vec<Player>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitorStatistics {
    pub average_ball_possession: f32,
    pub cards_given: u32,
    pub corner_kicks: u32,
    pub free_kicks: u32,
    pub goals_by_foot: u32,
    pub goals_by_head: u32,
    pub goals_conceded: u32,
    pub goals_conceded_first_half: u32,
    pub goals_conceded_second_half: u32,
    pub goals_scored: u32,
    pub goals_scored_first_half: u32,
    pub goals_scored_second_half: u32,
    pub matches_played: u32,
    pub offsides: u32,
    pub penalties_missed: Option<u32>,
    pub red_cards: u32,
    pub shots_blocked: u32,
    pub shots_off_target: u32,
    pub shots_on_bar: u32,
    pub shots_on_post: u32,
    pub shots_on_target: u32,
    pub shots_total: u32,
    pub yellow_cards: u32,
    pub yellow_red_cards: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStatisticsResponse {
    pub generated_at: String,
    pub season: SeasonDetails,
    pub competitor: CompetitorDetails,
}
