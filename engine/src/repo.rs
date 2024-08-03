use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Default)]
pub struct PlayerStatsRepo {
    data: Vec<PlayerStats>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Competition {
    pub id: String,
    pub name: String,
    pub location: String,
    pub gender: Gender,
    pub season_start: NaiveDate,
    pub season_end: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub abbreviation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Metric {
    GoalsScored { value: u32 },
    Assists { value: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub team: Team,
}

// TODO! optimize
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub player: Player,
    pub metrics: Vec<Metric>,
    pub competition: Competition,
}
