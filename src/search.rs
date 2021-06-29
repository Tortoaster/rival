use std::hash::Hash;

use crate::cache::Cached;
use crate::game::Game;

pub(crate) type Value = i32;

#[derive(Clone, Debug)]
pub(crate) struct SearchResult<M, const N: usize> {
    depth: u32,
    value: [Value; N],
    pub best: Option<M>,
}

pub(crate) trait Search<const N: usize> {
    type Move;

    fn max_n(&mut self, depth: u32, scores: &mut [Value; N]) -> SearchResult<Self::Move, N>;
}

impl<G: ?Sized + Game<N>, const N: usize> Search<N> for G {
    type Move = G::Move;

    fn max_n(&mut self, depth: u32, scores: &mut [Value; N]) -> SearchResult<Self::Move, N> {
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

            best
        }
    }
}

impl<G: Game<N> + Clone + Eq + Hash, const N: usize> Search<N> for Cached<G, N> where G::Move: Clone {
    type Move = G::Move;

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
