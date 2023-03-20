pub type Value = i16;

pub trait Evaluate<const N: usize> {
    fn turn(&self) -> usize;

    fn evaluate(&self) -> [Value; N];

    fn quiet(&self) -> bool {
        true
    }
}

pub trait SimpleEvaluate {
    fn second_players_turn(&self) -> bool;

    fn evaluate(&self) -> Value;

    fn quiet(&self) -> bool {
        true
    }
}

impl<G: SimpleEvaluate> Evaluate<2> for G {
    fn turn(&self) -> usize {
        self.second_players_turn() as usize
    }

    fn evaluate(&self) -> [Value; 2] {
        let value = self.evaluate();
        if self.second_players_turn() {
            [-value, value]
        } else {
            [value, -value]
        }
    }

    fn quiet(&self) -> bool {
        self.quiet()
    }
}
