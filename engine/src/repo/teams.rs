use super::in_memo::InMemoRepository;
use super::model::Team;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TeamsRepo {
    data: HashMap<Arc<String>, Team>,
}

impl InMemoRepository<Team> for TeamsRepo {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, value: Team) {
        self.data.insert(Arc::clone(&value.id), value);
    }

    fn insert(&mut self, key: Arc<String>, value: Team) {
        self.data.insert(key, value);
    }

    fn all(&self) -> &HashMap<Arc<String>, Team> {
        &self.data
    }
}
