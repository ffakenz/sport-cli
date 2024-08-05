use std::str::FromStr;

use engine::{
    engine::{Dimension, Sort},
    repo::model::{Gender, Metric},
};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SportArg {
    #[default]
    Football,
    // TODO! support other sports
}

impl FromStr for SportArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "football" => Ok(SportArg::Football),
            _ => Err(format!("Invalid sport: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DimensionArg {
    pub value: Dimension,
}
impl FromStr for DimensionArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "player" => Ok(DimensionArg {
                value: Dimension::Player,
            }),
            "team" => Ok(DimensionArg {
                value: Dimension::Team,
            }),
            _ => Err(format!("Invalid dimension: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MetricArg {
    pub value: Metric,
}
impl FromStr for MetricArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "score" => Ok(MetricArg {
                value: Metric::GoalsScored { value: 0 },
            }),
            "assist" => Ok(MetricArg {
                value: Metric::Assists { value: 0 },
            }),
            _ => Err(format!("Invalid metric: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GenderArg {
    pub value: Gender,
}
impl FromStr for GenderArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "male" => Ok(GenderArg {
                value: Gender::Male,
            }),
            "female" => Ok(GenderArg {
                value: Gender::Female,
            }),
            _ => Err(format!("Invalid gender: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SortArg {
    pub value: Sort,
}
impl FromStr for SortArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "asc" => Ok(SortArg { value: Sort::Asc }),
            "desc" => Ok(SortArg { value: Sort::Desc }),
            _ => Err(format!("Invalid sort: {}", s)),
        }
    }
}
