use super::in_memo::InMemoRepository;
use super::model::Player;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlayersRepo {
    data: HashMap<Arc<String>, Player>,
}

impl InMemoRepository<Player> for PlayersRepo {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, value: Player) {
        self.data.insert(Arc::clone(&value.id), value);
    }

    fn insert(&mut self, key: Arc<String>, value: Player) {
        self.data.insert(key, value);
    }

    fn all(&self) -> &HashMap<Arc<String>, Player> {
        &self.data
    }
}
