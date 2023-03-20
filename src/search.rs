use crate::{
    evaluate::{Evaluate, Value},
    moves::Moves,
    perform::Perform,
};

#[derive(Clone, Debug)]
struct SearchResult<M, const N: usize> {
    depth: u8,
    value: [Value; N],
    best: Option<M>,
}

impl<M, const N: usize> SearchResult<M, N> {
    const WORST: SearchResult<M, N> = SearchResult {
        depth: 0,
        value: [Value::MIN; N],
        best: None,
    };
}

trait Search<const N: usize>: Evaluate<N> + Moves {
    fn max_n(&mut self, depth: u8, scores: &mut [Value; N]) -> SearchResult<Self::Move, N>;
}

impl<G, const N: usize> Search<N> for G
where
    G: Evaluate<N> + Moves + Perform,
{
    fn max_n(&mut self, depth: u8, scores: &mut [Value; N]) -> SearchResult<Self::Move, N> {
        if (depth == 0 && self.quiet()) || self.moves().next().is_none() {
            SearchResult {
                depth: 0,
                value: self.evaluate(),
                best: None,
            }
        } else {
            let mut best = SearchResult::WORST;

            for m in self.moves() {
                let remember = self.perform(&m);
                let current = self.max_n(depth - 1, scores);
                self.revert(remember);

                let turn = self.turn();
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

pub trait SearchExt<const N: usize>: Moves {
    fn best_move(&mut self, depth: u8) -> Option<Self::Move>;
}

impl<G: Search<N>, const N: usize> SearchExt<N> for G {
    fn best_move(&mut self, depth: u8) -> Option<Self::Move> {
        self.max_n(depth, &mut [Value::MIN; N]).best
    }
}
