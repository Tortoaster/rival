use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

#[derive(Clone, Debug)]
pub struct TranspositionTable<S, V> {
    map: HashMap<u32, V>,
    phantom: PhantomData<S>,
}

impl<S, V> TranspositionTable<S, V> {
    pub fn new() -> Self {
        TranspositionTable {
            map: HashMap::new(),
            phantom: PhantomData,
        }
    }
}

impl<S: ZobristHash, V> TranspositionTable<S, V> {
    pub fn insert(&mut self, state: &S, value: V) {
        self.map.insert(state.hash(), value);
    }

    pub fn get(&self, state: &S) -> Option<&V> {
        self.map.get(&state.hash())
    }
}

impl<S, V> Default for TranspositionTable<S, V> {
    fn default() -> Self {
        TranspositionTable::new()
    }
}

pub trait ZobristHash {
    fn hash(&self) -> u32;
}

impl<T: Hash> ZobristHash for T {
    fn hash(&self) -> u32 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish() as u32
    }
}
