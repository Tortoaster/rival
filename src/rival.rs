use std::{
    marker::PhantomData,
    time::{Duration, Instant},
};

use crate::{
    cache::TranspositionTable,
    error::{RivalError, RivalResult},
    search::Strategy,
    Moves, Play, SearchResult,
};

#[derive(Debug, Default)]
pub struct Rival<G: Moves, S: Strategy<G, N, CAP>, const N: usize, const CAP: usize> {
    phantom: PhantomData<[(G, S); N]>,
    cache: TranspositionTable<G, SearchResult<S::Value, G::Move>, CAP>,
}

impl<G: Moves, S: Strategy<G, N, CAP>, const N: usize, const CAP: usize> Rival<G, S, N, CAP> {
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

    pub fn get_best_within(&mut self, game: &mut G, timeout: Duration) -> RivalResult<G::Move> {
        let start_time = Instant::now();
        let mut depth = 1;
        let mut best = None;

        while best.is_none() || start_time.elapsed() < timeout {
            let result = S::search(game, depth, &mut self.cache);
            best = Some(result.best.ok_or(RivalError::NoMove)?);
            depth += 1;
        }

        Ok(best.unwrap())
    }
}

impl<G: Moves + Play, S: Strategy<G, N, CAP>, const N: usize, const CAP: usize>
    Rival<G, S, N, CAP>
{
    pub fn play(&mut self, game: &mut G, depth: u8) -> RivalResult<()> {
        let best = self.get_best(game, depth)?;
        game.play(&best);

        Ok(())
    }

    pub fn play_within(&mut self, game: &mut G, timeout: Duration) -> RivalResult<()> {
        let best = self.get_best_within(game, timeout)?;
        game.play(&best);

        Ok(())
    }
}
