type Value = i32;

pub trait Game<const N: usize> {
    type Move;

    const DEPTH: u32;

    fn turn(&self) -> usize;

    fn evaluate(&self) -> [Value; N];

    fn get_moves(&self) -> Vec<Self::Move>;

    fn perform(&mut self, action: &Self::Move);

    fn undo(&mut self, action: &Self::Move);

    fn is_quiet(&self) -> bool {
        true
    }

    fn find_best(&mut self) -> Option<Self::Move> {
        self.max_n(Self::DEPTH, &mut [Value::MIN; N]).best
    }

    fn max_n(&mut self, depth: u32, scores: &mut [Value; N]) -> SearchResult<Self::Move, N> {
        let moves = self.get_moves();
        let turn = self.turn();

        if (depth <= 0 && self.is_quiet()) || moves.is_empty() {
            SearchResult {
                depth: 0,
                value: self.evaluate(),
                best: None
            }
        } else {
            let mut best = SearchResult {
                depth: 0,
                value: [Value::MIN; N],
                best: None
            };

            for m in moves {
                self.perform(&m);
                let current = self.max_n(depth - 1, scores);
                self.undo(&m);

                if current.value[turn] > best.value[turn] {
                    best = SearchResult {
                        depth: current.depth + 1,
                        value: current.value,
                        best: Some(m)
                    };
                }
            }

            best
        }
    }
}

#[derive(Clone)]
pub struct SearchResult<M, const N: usize> {
    depth: u32,
    value: [Value; N],
    best: Option<M>
}
