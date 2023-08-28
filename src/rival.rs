use std::marker::PhantomData;

use crate::{
    error::{RivalError, RivalResult},
    search::Strategy,
    Moves, Play,
};

#[derive(Copy, Clone, Debug, Default)]
pub struct Rival<G, S, const N: usize> {
    phantom: PhantomData<[(G, S); N]>,
}

impl<G, S, const N: usize> Rival<G, S, N> {
    pub fn new() -> Self {
        Rival {
            phantom: PhantomData,
        }
    }
}

impl<G: Moves, S: Strategy<G, N>, const N: usize> Rival<G, S, N> {
    pub fn get_best(&self, game: &mut G, depth: u8) -> RivalResult<G::Move> {
        S::search(game, depth).best.ok_or(RivalError::NoMove)
    }
}

impl<G: Moves + Play, S: Strategy<G, N>, const N: usize> Rival<G, S, N> {
    pub fn play(&self, game: &mut G, depth: u8) -> RivalResult<()> {
        let best = self.get_best(game, depth)?;
        game.play(&best);

        Ok(())
    }
}
