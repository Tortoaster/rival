use std::marker::PhantomData;

use crate::{
    error::{RivalError, RivalResult},
    search::Strategy,
    Moves, Play, Value,
};

#[derive(Copy, Clone, Debug)]
pub struct Rival<G, S, const N: usize> {
    strategy: S,
    phantom: PhantomData<[G; N]>,
}

impl<G, S, const N: usize> Rival<G, S, N> {
    pub fn new(strategy: S) -> Self {
        Rival {
            strategy,
            phantom: PhantomData,
        }
    }
}

impl<G: Moves, S: Strategy<G, N>, const N: usize> Rival<G, S, N> {
    pub fn get_best(&mut self, game: &mut G, depth: u8) -> RivalResult<G::Move> {
        unsafe {
            self.strategy
                .search(game, depth, &mut [Value::MIN; N])
                .best
                .ok_or(RivalError::NoMove)
        }
    }
}

impl<G: Moves + Play, S: Strategy<G, N>, const N: usize> Rival<G, S, N> {
    pub fn play(&mut self, game: &mut G, depth: u8) -> RivalResult<()> {
        let best = self.get_best(game, depth)?;
        game.play(&best);

        Ok(())
    }
}
