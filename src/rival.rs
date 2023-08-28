use std::marker::PhantomData;

use crate::{
    cache::TranspositionTable,
    error::{RivalError, RivalResult},
    search::Strategy,
    Moves, Play, SearchResult,
};

#[derive(Clone, Debug, Default)]
pub struct Rival<G: Moves, S: Strategy<G, N>, const N: usize> {
    phantom: PhantomData<[(G, S); N]>,
    cache: TranspositionTable<G, SearchResult<S::Value, G::Move>>,
}

impl<G: Moves, S: Strategy<G, N>, const N: usize> Rival<G, S, N> {
    pub fn new() -> Self {
        Rival {
            phantom: PhantomData,
            cache: TranspositionTable::new(),
        }
    }

    pub fn get_best(&mut self, game: &mut G, depth: u8) -> RivalResult<G::Move> {
        S::search(game, depth, &mut self.cache)
            .best
            .ok_or(RivalError::NoMove)
    }
}

impl<G: Moves + Play, S: Strategy<G, N>, const N: usize> Rival<G, S, N> {
    pub fn play(&mut self, game: &mut G, depth: u8) -> RivalResult<()> {
        let best = self.get_best(game, depth)?;
        game.play(&best);

        Ok(())
    }
}
