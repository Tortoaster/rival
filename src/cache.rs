use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

#[derive(Debug)]
pub struct TranspositionTable<K, V, const CAP: usize> {
    items: Box<[Option<V>]>,
    phantom: PhantomData<K>,
}

impl<K, V, const CAP: usize> TranspositionTable<K, V, CAP> {
    pub fn new() -> Self {
        TranspositionTable {
            items: Box::new([(); CAP].map(|_| None)),
            phantom: Default::default(),
        }
    }
}

impl<K: ZobristHash, V, const CAP: usize> TranspositionTable<K, V, CAP> {
    pub fn insert(&mut self, key: &K, value: V) {
        let index = key.zobrist_hash() % CAP;
        self.items[index] = Some(value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = key.zobrist_hash() % CAP;
        self.items[index].as_ref()
    }
}

impl<K, V, const CAP: usize> Default for TranspositionTable<K, V, CAP> {
    fn default() -> Self {
        TranspositionTable::new()
    }
}

pub trait ZobristHash {
    fn zobrist_hash(&self) -> usize;
}

pub trait LazyZobristHash: Hash {}

impl<T: LazyZobristHash> ZobristHash for T {
    #[inline]
    fn zobrist_hash(&self) -> usize {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish() as usize
    }
}
