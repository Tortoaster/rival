use crate::{
    cache::{TranspositionTable, ZobristHash},
    search::SearchResult,
    Evaluate, EvaluateZeroSum, Moves, Play, Strategy, Value,
};

#[derive(Copy, Clone, Debug)]
pub struct Negamax;

impl Negamax {
    fn search_alpha_beta<S: EvaluateZeroSum + Play + Moves + ZobristHash, const CAP: usize>(
        state: &mut S,
        depth: u8,
        mut alpha: Value,
        beta: Value,
        cache: &mut TranspositionTable<
            S,
            SearchResult<<Negamax as Strategy<S, 2, CAP>>::Value, S::Move>,
            CAP,
        >,
    ) -> SearchResult<<Negamax as Strategy<S, 2, CAP>>::Value, S::Move>
    where
        S::Move: Copy,
    {
        if let Some(result) = cache.get(state) {
            if result.depth >= depth {
                return *result;
            }
        }

        let best = if state.moves().next().is_none() {
            SearchResult {
                depth: u8::MAX,
                value: if state.min_turn() {
                    -state.evaluate()
                } else {
                    state.evaluate()
                },
                best: None,
            }
        } else if depth == 0 && state.quiet() {
            SearchResult {
                depth: 0,
                value: if state.min_turn() {
                    -state.evaluate()
                } else {
                    state.evaluate()
                },
                best: None,
            }
        } else {
            let mut best = SearchResult::MIN;

            let state_ptr: *mut S = state;
            for m in state.moves() {
                // Safety: as long as unplay properly restores any existing references that play
                // destroys, this should be safe, right?
                let next_state = unsafe { &mut *state_ptr };
                let turn = state.turn();
                let remember = next_state.play(&m);
                let current = if turn == next_state.turn() {
                    Self::search_alpha_beta(next_state, depth - 1, alpha, beta, cache)
                } else {
                    -Self::search_alpha_beta(next_state, depth - 1, -beta, -alpha, cache)
                };
                next_state.unplay(remember);

                if current.value > best.value {
                    alpha = alpha.max(current.value);
                    best = SearchResult {
                        depth: current.depth.saturating_add(1),
                        value: current.value,
                        best: Some(m),
                    };
                }

                if alpha >= beta {
                    break;
                }
            }

            best
        };

        cache.insert(state, best);

        best
    }
}

impl<S: EvaluateZeroSum + Play + Moves + ZobristHash, const CAP: usize> Strategy<S, 2, CAP>
    for Negamax
where
    S::Move: Copy,
{
    type Value = Value;

    fn search(
        state: &mut S,
        depth: u8,
        cache: &mut TranspositionTable<S, SearchResult<Self::Value, S::Move>, CAP>,
    ) -> SearchResult<Self::Value, S::Move> {
        let alpha = Value::MIN + 1;
        let beta = Value::MAX;

        Self::search_alpha_beta(state, depth, alpha, beta, cache)
    }
}
