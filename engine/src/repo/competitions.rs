use super::in_memo::InMemoRepository;
use super::model::Competition;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompetitionsRepo {
    data: HashMap<Arc<String>, Competition>,
}

impl InMemoRepository<Competition> for CompetitionsRepo {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, value: Competition) {
        self.data.insert(Arc::clone(&value.id), value);
    }

    fn insert(&mut self, key: Arc<String>, value: Competition) {
        self.data.insert(key, value);
    }

    fn all(&self) -> &HashMap<Arc<String>, Competition> {
        &self.data
    }
}
