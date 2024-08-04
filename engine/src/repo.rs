use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlayerStatsRepo {
    data: Vec<PlayerStats>,
}
unsafe impl Send for PlayerStatsRepo {}
unsafe impl Sync for PlayerStatsRepo {}

impl PlayerStatsRepo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, player_stats: PlayerStats) {
        self.data.push(player_stats);
    }

    pub fn all(&self) -> &Vec<PlayerStats> {
        &self.data
    }
}

// --------------------------------------------------
// Model for "player's statistics" repo
// --------------------------------------------------

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    #[default]
    Male,
    Female,
}
unsafe impl Send for Gender {}
unsafe impl Sync for Gender {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Competition {
    pub id: String,
    pub name: String,
    pub location: String,
    pub gender: Gender,
    pub season_start: NaiveDate,
    pub season_end: NaiveDate,
}
unsafe impl Send for Competition {}
unsafe impl Sync for Competition {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub abbreviation: String,
}
unsafe impl Send for Team {}
unsafe impl Sync for Team {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Metric {
    #[serde(rename = "goals_scored")]
    GoalsScored {
        value: u32,
    },
    Assists {
        value: u32,
    },
}
unsafe impl Send for Metric {}
unsafe impl Sync for Metric {}

impl Default for Metric {
    fn default() -> Self {
        Metric::GoalsScored { value: 0 }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub team: Team,
}
unsafe impl Send for Player {}
unsafe impl Sync for Player {}

// TODO! optimize
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PlayerStats {
    pub player: Player,
    pub metrics: Vec<Metric>,
    pub competition: Competition,
}
unsafe impl Send for PlayerStats {}
unsafe impl Sync for PlayerStats {}
