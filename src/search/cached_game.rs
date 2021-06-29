use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

use crate::search::{Game, SearchResult, Value};

#[derive(Clone, Debug)]
pub struct Cached<G: Game<N>, const N: usize> {
    game: G,
    cache: HashMap<G, SearchResult<G::Move, N>>,
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

impl<G: Game<N> + Clone + Eq + Hash, const N: usize> Game<N> for Cached<G, N> where G::Move: Clone {
    type Move = G::Move;

    const DEPTH: u32 = G::DEPTH;

    #[inline]
    fn turn(&self) -> usize {
        self.game.turn()
    }

    #[inline]
    fn value(&self) -> [i32; N] {
        self.game.value()
    }

    #[inline]
    fn moves(&self) -> Vec<Self::Move> {
        self.game.moves()
    }

    #[inline]
    fn perform(&mut self, m: &Self::Move) {
        self.game.perform(m)
    }

    #[inline]
    fn revert(&mut self, m: &Self::Move) {
        self.game.revert(m)
    }

    fn max_n(&mut self, depth: u32, scores: &mut [Value; N]) -> SearchResult<Self::Move, N> {
        if let Some(result) = self.cache.get(&self.game) {
            if result.depth >= depth {
                return result.clone();
            }
        }

        let moves = self.moves();
        let turn = self.turn();

        if (depth <= 0 && self.quiet()) || moves.is_empty() {
            SearchResult {
                depth: 0,
                value: self.value(),
                best: None,
            }
        } else {
            let mut best = SearchResult {
                depth: 0,
                value: [Value::MIN; N],
                best: None,
            };

            for m in moves {
                self.perform(&m);
                let current = self.max_n(depth - 1, scores);
                self.revert(&m);

                if current.value[turn] > best.value[turn] {
                    best = SearchResult {
                        depth: current.depth + 1,
                        value: current.value,
                        best: Some(m),
                    };
                }
            }

            self.cache.insert(self.game.clone(), best.clone());

            best
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
