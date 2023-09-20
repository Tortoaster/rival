use std::ops::Neg;

pub use max_n::MaxN;
pub use negamax::Negamax;

use crate::{cache::TranspositionTable, moves::Moves, Value};

mod max_n;
mod negamax;

pub trait Strategy<S: Moves, const N: usize, const CAP: usize> {
    type Value: HasMin;

    fn search(
        state: &mut S,
        depth: u8,
        cache: &mut TranspositionTable<S, SearchResult<Self::Value, S::Move>, CAP>,
    ) -> SearchResult<Self::Value, S::Move>;
}

#[derive(Copy, Clone, Debug)]
pub struct SearchResult<V, M> {
    pub depth: u8,
    pub value: V,
    pub best: Option<M>,
}

impl<V: HasMin, M> SearchResult<V, M> {
    const MIN: SearchResult<V, M> = SearchResult {
        depth: 0,
        value: V::MIN,
        best: None,
    };
}

impl<V: Neg<Output = V>, M> Neg for SearchResult<V, M> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.value = -self.value;
        self
    }
}

pub trait HasMin {
    const MIN: Self;
}

impl HasMin for Value {
    // Plus one to prevent overflow when negating
    const MIN: Self = Value::MIN + 1;
}

impl<const N: usize> HasMin for [Value; N] {
    // Plus one to prevent overflow when negating
    const MIN: Self = [Value::MIN + 1; N];
}
