use std::collections::HashMap;
use std::sync::Arc;

pub trait InMemoRepository<V: Default> {
    fn new() -> Self
    where
        Self: Default;

    fn push(&mut self, value: V);
    fn insert(&mut self, key: Arc<String>, value: V);
    fn all(&self) -> &HashMap<Arc<String>, V>;

    /// Returns an iterator that filters values based on a predicate function.
    /// The iterator produces a set of references to the values that satisfy the predicate.
    fn filter_iter<'a, F>(&'a self, predicate: F) -> impl Iterator<Item = (Arc<String>, &V)>
    where
        F: Fn(&V) -> bool + 'a,
        V: 'a,
    {
        self.all().iter().filter_map(move |(key, value)| {
            if predicate(value) {
                Some((Arc::clone(key), value))
            } else {
                None
            }
        })
    }

    /// Finds a value by its key.
    fn find(&self, key: &Arc<String>) -> Option<&V> {
        self.all().get(key)
    }

    /// Returns an iterator that maps values based on a mapping function.
    /// The iterator produces a new HashMap where each value is transformed by the mapping function.
    fn map_iter<'a, F, W>(&'a self, mapper: F) -> impl Iterator<Item = (Arc<String>, W)>
    where
        F: Fn(&V) -> W + 'a,
        V: 'a,
        W: 'a,
    {
        self.all()
            .iter()
            .map(move |(key, value)| (Arc::clone(key), mapper(value)))
    }
}
