use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

use crate::search::{Game, SearchResult, Search, Value};

#[derive(Clone, Debug)]
pub struct Cached<G: Game<N>, const N: usize> {
    pub(crate) game: G,
    pub(crate) cache: HashMap<G, SearchResult<G::Move, N>>,
}

impl<G: Game<N> + Clone + Eq + Hash, const N: usize> Cached<G, N> where G::Move: Clone {
    #[doc(hidden)]
    pub fn best_move(&mut self) -> Option<G::Move> {
        self.max_n(G::DEPTH, &mut [Value::MIN; N]).best
    }
}

/// This trait provides the [`WithCache::with_cache`] method, which is implemented for all types
/// that implement [`Game`] and satisfy the trait bounds necessary to store intermediate results in
/// cache.
pub trait WithCache<const N: usize>: Game<N> + Sized {
    /// Enables this game to use a cache with the specified [`size`].
    ///
    /// This can improve performance by storing evaluations of game positions that have already been
    /// searched.
    fn with_cache(self, size: usize) -> Cached<Self, N>;
}

impl<G: Game<N> + Clone + Eq + Hash, const N: usize> WithCache<N> for G where G::Move: Clone {
    fn with_cache(self, size: usize) -> Cached<Self, N> {
        Cached {
            game: self,
            cache: HashMap::with_capacity(size),
        }
    }
}

impl<G: Game<N>, const N: usize> Deref for Cached<G, N> {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        &self.game
    }
}

impl<G: Game<N>, const N: usize> DerefMut for Cached<G, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.game
    }
}

impl<G: Game<N> + Display, const N: usize> fmt::Display for Cached<G, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.game)
    }
}
