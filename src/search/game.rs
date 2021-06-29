use crate::search::{SearchResult, Value};

pub trait Game<const N: usize> {
    type Move;

    const DEPTH: u32;

    fn turn(&self) -> usize;

    fn value(&self) -> [Value; N];

    fn moves(&self) -> Vec<Self::Move>;

    fn perform(&mut self, action: &Self::Move);

    fn undo(&mut self, action: &Self::Move);

    fn best_move(&mut self) -> Option<Self::Move> {
        self.max_n(Self::DEPTH, &mut [Value::MIN; N]).best
    }

    fn max_n(&mut self, depth: u32, scores: &mut [Value; N]) -> SearchResult<Self::Move, N> {
        let moves = self.moves();
        let turn = self.turn();

        if (depth <= 0 && self.quiet()) || moves.is_empty() {
            SearchResult {
                depth: 0,
                value: self.value(),
                best: None,
            }
        } else {
            let mut best = SearchResult {
                depth: 0,
                value: [Value::MIN; N],
                best: None,
            };

            for m in moves {
                self.perform(&m);
                let current = self.max_n(depth - 1, scores);
                self.undo(&m);

                if current.value[turn] > best.value[turn] {
                    best = SearchResult {
                        depth: current.depth + 1,
                        value: current.value,
                        best: Some(m),
                    };
                }
            }

            best
        }
    }

    fn quiet(&self) -> bool {
        true
    }
}
