use crate::{
    search::{SearchResult, Strategy},
    Evaluate, Moves, Play, Value,
};

#[derive(Copy, Clone, Debug)]
pub struct MaxN;

impl<S: Evaluate<N> + Play + Moves, const N: usize> Strategy<S, N> for MaxN {
    type Value = [Value; N];

    fn search(state: &mut S, depth: u8) -> SearchResult<Self::Value, S::Move> {
        if state.moves().next().is_none() {
            SearchResult {
                depth: u8::MAX,
                value: state.evaluate(),
                best: None,
            }
        } else if depth == 0 && state.quiet() {
            SearchResult {
                depth: 0,
                value: state.evaluate(),
                best: None,
            }
        } else {
            let mut best = SearchResult::<Self::Value, S::Move>::MIN;

            let state_ptr: *mut S = state;
            for m in state.moves() {
                // Safety: as long as unplay properly restores any existing references that play
                // destroys, this should be safe, right?
                let next_state = unsafe { &mut *state_ptr };
                let remember = next_state.play(&m);
                let current = Self::search(next_state, depth - 1);
                next_state.unplay(remember);

                let turn = state.turn();
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
