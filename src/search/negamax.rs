use crate::{search::SearchResult, Evaluate, Moves, Play, Strategy, Value};

#[derive(Copy, Clone, Debug)]
pub struct Negamax;

impl Negamax {
    fn search_alpha_beta<S: Evaluate<2> + Play + Moves>(
        state: &mut S,
        depth: u8,
        mut alpha: Value,
        beta: Value,
    ) -> SearchResult<<Negamax as Strategy<S, 2>>::Value, S::Move> {
        if state.moves().next().is_none() {
            SearchResult {
                depth: u8::MAX,
                value: state.evaluate()[state.turn()],
                best: None,
            }
        } else if depth == 0 && state.quiet() {
            SearchResult {
                depth: 0,
                value: state.evaluate()[state.turn()],
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
                    Self::search_alpha_beta(next_state, depth - 1, alpha, beta)
                } else {
                    -Self::search_alpha_beta(next_state, depth - 1, -beta, -alpha)
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
        }
    }
}

impl<S: Evaluate<2> + Play + Moves> Strategy<S, 2> for Negamax {
    type Value = Value;

    fn search(state: &mut S, depth: u8) -> SearchResult<Self::Value, S::Move> {
        let alpha = Value::MIN + 1;
        let beta = Value::MAX;

        Self::search_alpha_beta(state, depth, alpha, beta)
    }
}
