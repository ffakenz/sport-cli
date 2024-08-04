use serde_derive::{Deserialize, Serialize};

// --------------------------------------------------
// Model for the "competitions" endpoint
// --------------------------------------------------
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CompetitionCategory {
    pub id: String,
    pub name: String,
}
unsafe impl Send for CompetitionCategory {}
unsafe impl Sync for CompetitionCategory {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompetitionGender {
    #[default]
    Men,
    Women,
}
unsafe impl Send for CompetitionGender {}
unsafe impl Sync for CompetitionGender {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Competition {
    pub id: String,
    pub name: String,
    pub gender: Option<CompetitionGender>,
    pub category: CompetitionCategory,
}
unsafe impl Send for Competition {}
unsafe impl Sync for Competition {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CompetitionsResponse {
    pub generated_at: String,
    pub competitions: Vec<Competition>,
}
unsafe impl Send for CompetitionsResponse {}
unsafe impl Sync for CompetitionsResponse {}

// --------------------------------------------------
// Model for the "competition_seasons" endpoint
// --------------------------------------------------
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Season {
    pub id: String,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub year: String,
    pub competition_id: String,
}
unsafe impl Send for Season {}
unsafe impl Sync for Season {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SeasonsResponse {
    pub generated_at: String,
    pub seasons: Vec<Season>,
}
unsafe impl Send for SeasonsResponse {}
unsafe impl Sync for SeasonsResponse {}

// --------------------------------------------------
// Model for the "season_competitors" endpoint
// --------------------------------------------------
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Competitor {
    pub id: String,
    pub name: String,
    pub short_name: String,
    pub abbreviation: String,
}
unsafe impl Send for Competitor {}
unsafe impl Sync for Competitor {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CompetitorsResponse {
    pub generated_at: String,
    pub season_competitors: Vec<Competitor>,
}
unsafe impl Send for CompetitorsResponse {}
unsafe impl Sync for CompetitorsResponse {}

// --------------------------------------------------
// Model for the "seasonal_competitor_statistics" endpoint
// --------------------------------------------------
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Sport {
    pub id: String,
    pub name: String,
}
unsafe impl Send for Sport {}
unsafe impl Sync for Sport {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SeasonDetails {
    pub id: String,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub year: String,
    pub competition_id: String,
    pub sport: Sport,
}
unsafe impl Send for SeasonDetails {}
unsafe impl Sync for SeasonDetails {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
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
unsafe impl Send for PlayerStatistics {}
unsafe impl Sync for PlayerStatistics {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub statistics: PlayerStatistics,
}
unsafe impl Send for Player {}
unsafe impl Sync for Player {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompetitorGender {
    #[default]
    Male,
    Female,
}
unsafe impl Send for CompetitorGender {}
unsafe impl Sync for CompetitorGender {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
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
unsafe impl Send for CompetitorDetails {}
unsafe impl Sync for CompetitorDetails {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
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
unsafe impl Send for CompetitorStatistics {}
unsafe impl Sync for CompetitorStatistics {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PlayerStatisticsResponse {
    pub generated_at: String,
    pub season: SeasonDetails,
    pub competitor: CompetitorDetails,
}
unsafe impl Send for PlayerStatisticsResponse {}
unsafe impl Sync for PlayerStatisticsResponse {}
