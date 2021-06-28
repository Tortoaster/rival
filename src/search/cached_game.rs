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

pub trait WithCache<const N: usize>: Game<N> + Sized {
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
    fn evaluate(&self) -> [i32; N] {
        self.game.evaluate()
    }

    #[inline]
    fn get_moves(&self) -> Vec<Self::Move> {
        self.game.get_moves()
    }

    #[inline]
    fn perform(&mut self, action: &Self::Move) {
        self.game.perform(action)
    }

    #[inline]
    fn undo(&mut self, action: &Self::Move) {
        self.game.undo(action)
    }

    fn max_n(&mut self, depth: u32, scores: &mut [Value; N]) -> SearchResult<Self::Move, N> {
        if let Some(result) = self.cache.get(&self.game) {
            if result.depth >= depth {
                return result.clone();
            }
        }

        let moves = self.get_moves();
        let turn = self.turn();

        if (depth <= 0 && self.is_quiet()) || moves.is_empty() {
            SearchResult {
                depth: 0,
                value: self.evaluate(),
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
                self.undo(&m);

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
