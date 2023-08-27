use crate::search::{SearchResult, Strategy};
use crate::{Evaluate, Moves, Play, Value};

pub struct MaxN;

impl<G: Evaluate<N> + Play + Moves, const N: usize> Strategy<G, N> for MaxN {
    fn search(
        &mut self,
        game: &mut G,
        depth: u8,
        scores: &mut [Value; N],
    ) -> SearchResult<G::Move, N> {
        if (depth == 0 && game.quiet()) || game.moves().next().is_none() {
            SearchResult {
                depth: 0,
                value: game.evaluate(),
                best: None,
            }
        } else {
            let mut best = SearchResult::WORST;

            for m in game.moves() {
                let remember = game.play(&m);
                let current = self.search(game, depth - 1, scores);
                game.unplay(remember);

                let turn = game.turn();
                if current.value[turn] > best.value[turn] {
                    best = SearchResult {
                        depth: current.depth.saturating_add(1),
                        value: current.value,
                        best: Some(m),
                    };
                }
            }

            best
        }
    }
}
