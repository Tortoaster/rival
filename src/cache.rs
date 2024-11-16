use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Debug)]
pub struct TranspositionTable<G: CacheKey, V, const CAP: usize> {
    items: Box<[Option<(G::Key, V)>]>,
}

impl<G: CacheKey, V, const CAP: usize> TranspositionTable<G, V, CAP> {
    pub fn new() -> Self {
        TranspositionTable {
            items: Box::new([(); CAP].map(|_| None)),
        }
    }
}

impl<G: ZobristHash + CacheKey, V, const CAP: usize> TranspositionTable<G, V, CAP> {
    pub fn insert(&mut self, state: &G, value: V) {
        let index = state.zobrist_hash() % CAP;
        let key = state.cache_key();
        self.items[index] = Some((key, value));
    }

    pub fn get(&self, state: &G) -> Option<&V> {
        let index = state.zobrist_hash() % CAP;
        match self.items[index].as_ref() {
            Some((old_key, value)) if state.cache_key() == *old_key => Some(value),
            _ => None,
        }
    }
}

impl<G: CacheKey, V, const CAP: usize> Default for TranspositionTable<G, V, CAP> {
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

/// Because hash collisions in these search algorithms are usually very common,
/// we need some way to determine whether two states with equal hashes indeed
/// represent the same state. Otherwise, a particularly good or bad score from
/// an earlier unrelated state might negatively impact the outcome of the
/// current search.
///
/// To simplify implementations, this crate provides two default options:
///
/// * [`CloneCacheKey`]: Copies the entire state to the cache entry. Will never
///   pick incorrect earlier entries, but can have a negative impact on
///   performance.
/// * [`IgnoreCacheKey`]: Assumes collisions do not occur, or are infrequent
///   enough to make little difference. Most performant option, but may result
///   in bad moves.
pub trait CacheKey {
    type Key: Eq;

    fn cache_key(&self) -> Self::Key;
}

/// Copies the entire state to the cache entry. Will never pick incorrect
/// earlier entries, but can have a negative impact on performance.
///
/// See [`CacheKey`] for more information.
pub trait CloneCacheKey: Clone {}

impl<T: CloneCacheKey + Eq> CacheKey for T {
    type Key = T;

    fn cache_key(&self) -> Self::Key {
        self.clone()
    }
}

// pub trait IgnoreCacheKey {}
//
// impl<T: CloneCacheKey> CacheKey for T {
//     type Key = ();
//
//     fn cache_key(&self) -> Self::Key {
//         ()
//     }
// }
