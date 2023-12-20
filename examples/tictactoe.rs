#![feature(test)]

use rival::{EvaluateZeroSum, LazyZobristHash, Moves, PlayClone, Value};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TicTacToe {
    present: u16,
    symbol: u16,
}

impl TicTacToe {
    const TOP_ROW: u16 = 0b1110000000000000;
    const MIDDLE_ROW: u16 = 0b0001110000000000;
    const BOTTOM_ROW: u16 = 0b0000001110000000;
    const LEFT_COLUMN: u16 = 0b1001001000000000;
    const CENTER_COLUMN: u16 = 0b0100100100000000;
    const RIGHT_COLUMN: u16 = 0b0010010010000000;
    const MAIN_DIAGONAL: u16 = 0b1000100010000000;
    const OFF_DIAGONAL: u16 = 0b0010101000000000;

    const TRIPLETS: [u16; 8] = [
        Self::TOP_ROW,
        Self::MIDDLE_ROW,
        Self::BOTTOM_ROW,
        Self::LEFT_COLUMN,
        Self::CENTER_COLUMN,
        Self::RIGHT_COLUMN,
        Self::MAIN_DIAGONAL,
        Self::OFF_DIAGONAL,
    ];

    const TURN_BIT: u16 = 0b0000000000000001;

    pub fn new() -> Self {
        TicTacToe {
            present: 0,
            symbol: 0,
        }
    }

    #[inline]
    const fn turn(&self) -> u16 {
        self.present & Self::TURN_BIT
    }

    #[inline]
    fn switch_turn(&mut self) {
        self.present ^= Self::TURN_BIT;
    }

    #[inline]
    fn place_unchecked(&mut self, m: u16) {
        self.present ^= m;
        if self.turn() == 1 {
            self.symbol ^= m;
        }
        self.switch_turn();
    }

    #[inline]
    const fn valid_moves(&self) -> u16 {
        !self.present & 0b1111111110000000
    }
}

impl EvaluateZeroSum for TicTacToe {
    fn min_turn(&self) -> bool {
        self.turn() == 1
    }

    fn evaluate(&self) -> Value {
        TicTacToe::TRIPLETS
            .iter()
            .find_map(|triplet| {
                if self.present & *triplet == *triplet {
                    let symbols = self.symbol & *triplet;
                    if symbols == 0 {
                        Some(1)
                    } else if symbols == 0xffff & *triplet {
                        Some(-1)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .unwrap_or(0)
    }
}

impl Moves for TicTacToe {
    type Move = u16;
    type Iter<'a> = <Vec<Self::Move> as IntoIterator>::IntoIter;

    fn moves(&self) -> Self::Iter<'static> {
        let mut turns = Vec::new();
        let valid = self.valid_moves();

        for index in 0u16..9 {
            let turn = 1u16 << (15 - index);
            if turn & valid != 0 {
                turns.push(turn);
            }
        }

        turns.into_iter()
    }
}

impl PlayClone for TicTacToe {
    fn play(&mut self, m: &Self::Move) {
        self.place_unchecked(*m);
    }
}

impl LazyZobristHash for TicTacToe {}

#[cfg(test)]
mod tests {
    extern crate test;

    use rival::{Evaluate, MaxN, Moves, Negamax, Rival};
    use test::Bencher;

    use crate::TicTacToe;

    /// Capacity of the transposition table of computer players in these tests.
    const CAP: usize = 7643;

    #[test]
    fn test_tictactoe_maxn() {
        let mut game = TicTacToe::new();
        let mut rival: Rival<_, MaxN, 2, CAP> = Rival::new();

        for _ in 0..9 {
            rival.play(&mut game, 9).unwrap();
        }

        assert_eq!(game.moves().len(), 0);
        assert_eq!(game.evaluate(), [0, 0]);
    }

    #[test]
    fn test_tictactoe_negamax() {
        let mut game = TicTacToe::new();
        let mut rival: Rival<_, Negamax, 2, CAP> = Rival::new();

        for _ in 0..9 {
            rival.play(&mut game, 9).unwrap();
        }

        assert_eq!(game.moves().len(), 0);
        assert_eq!(game.evaluate(), [0, 0]);
    }

    #[test]
    fn test_tictactoe_maxn_vs_negamax() {
        // The game results in a tie if both players play optimally, which will make
        // other tests pass. However, if both players actively evade winning, the game
        // will also result in a tie. This test makes sure `Negamax` doesn't do that, as
        // `MaxN` was manually verified to be working properly.
        let mut game = TicTacToe::new();
        let mut a: Rival<_, Negamax, 2, CAP> = Rival::new();
        let mut b: Rival<_, MaxN, 2, CAP> = Rival::new();

        for _ in 0..4 {
            a.play(&mut game, 9).unwrap();
            b.play(&mut game, 9).unwrap();
        }
        a.play(&mut game, 9).unwrap();

        assert_eq!(game.moves().len(), 0);
        assert_eq!(game.evaluate(), [0, 0]);
    }

    #[bench]
    fn bench_tictactoe_maxn(bencher: &mut Bencher) {
        let mut rival: Rival<_, MaxN, 2, CAP> = Rival::new();

        bencher.iter(|| {
            let mut game = TicTacToe::new();

            for _ in 0..9 {
                rival.play(&mut game, 9).unwrap();
            }
        });
    }

    #[bench]
    fn bench_tictactoe_negamax(bencher: &mut Bencher) {
        let mut rival: Rival<_, Negamax, 2, CAP> = Rival::new();

        bencher.iter(|| {
            let mut game = TicTacToe::new();

            for _ in 0..9 {
                rival.play(&mut game, 9).unwrap();
            }
        });
    }
}
