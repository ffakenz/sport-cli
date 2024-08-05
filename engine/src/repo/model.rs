use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;

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
    pub id: Arc<String>,
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
    pub id: Arc<String>,
    pub name: String,
    pub abbreviation: String,
}
unsafe impl Send for Team {}
unsafe impl Sync for Team {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, PartialOrd, Eq, Ord)]
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

impl Metric {
    pub fn value(&self) -> u32 {
        match self {
            Metric::GoalsScored { value } => *value,
            Metric::Assists { value } => *value,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub id: Arc<String>,
    pub name: String,
}
unsafe impl Send for Player {}
unsafe impl Sync for Player {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PlayerStats {
    pub player_id: Arc<String>,
    pub team_id: Arc<String>,
    pub competition_id: Arc<String>,
    pub metrics: Vec<Metric>,
}
unsafe impl Send for PlayerStats {}
unsafe impl Sync for PlayerStats {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PlayerDetails<'a> {
    pub player_id: &'a str,
    pub player_name: &'a str,
    pub team_id: &'a str,
    pub team_name: &'a str,
    pub competition_id: &'a str,
    pub competition_name: &'a str,
}
unsafe impl<'a> Send for PlayerDetails<'a> {}
unsafe impl<'a> Sync for PlayerDetails<'a> {}
