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

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Competition {
    pub id: Arc<String>,
    pub name: String,
    pub location: String,
    pub gender: Gender,
    pub season_start: NaiveDate,
    pub season_end: NaiveDate,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Team {
    pub id: Arc<String>,
    pub name: String,
    pub abbreviation: String,
}

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

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PlayerStats {
    pub player_id: Arc<String>,
    pub team_id: Arc<String>,
    pub competition_id: Arc<String>,
    pub metrics: Vec<Metric>,
}

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
