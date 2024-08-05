use super::in_memo::InMemoRepository;
use super::model::PlayerStats;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlayerStatsRepo {
    data: HashMap<Arc<String>, PlayerStats>,
}

impl InMemoRepository<PlayerStats> for PlayerStatsRepo {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, value: PlayerStats) {
        let player_id = Arc::clone(&value.player_id).to_string();
        let team_id = Arc::clone(&value.team_id).to_string();
        let competition_id = Arc::clone(&value.competition_id).to_string();

        let key = format!("{}::{}::{}", player_id, team_id, competition_id);
        self.data.insert(Arc::new(key), value);
    }

    fn insert(&mut self, key: Arc<String>, value: PlayerStats) {
        self.data.insert(key, value);
    }

    fn all(&self) -> &HashMap<Arc<String>, PlayerStats> {
        &self.data
    }
}
