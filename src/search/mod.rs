use crate::{evaluate::Value, moves::Moves};

mod max_n;

pub use max_n::MaxN;

pub trait Strategy<G: Moves, const N: usize> {
    unsafe fn search(
        &self,
        game_ptr: *mut G,
        depth: u8,
        scores: &mut [Value; N],
    ) -> SearchResult<G::Move, N>;
}

#[derive(Copy, Clone, Debug)]
pub struct SearchResult<M, const N: usize> {
    pub depth: u8,
    pub value: [Value; N],
    pub best: Option<M>,
}

impl<M, const N: usize> SearchResult<M, N> {
    const WORST: SearchResult<M, N> = SearchResult {
        depth: 0,
        value: [Value::MIN; N],
        best: None,
    };
}
