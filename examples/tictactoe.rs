#![feature(test)]

use rival::{Evaluate, Moves, PerformWithClone, Value};

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

impl Evaluate<2> for TicTacToe {
    fn turn(&self) -> usize {
        self.turn() as usize
    }

    fn evaluate(&self) -> [Value; 2] {
        TicTacToe::TRIPLETS
            .iter()
            .find_map(|triplet| {
                if self.present & *triplet == *triplet {
                    let symbols = self.symbol & *triplet;
                    if symbols == 0 {
                        Some([1, -1])
                    } else if symbols == 0xffff & *triplet {
                        Some([-1, 1])
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .unwrap_or([0, 0])
    }
}

impl Moves for TicTacToe {
    type Move = u16;
    type Iter = <Vec<Self::Move> as IntoIterator>::IntoIter;

    fn moves(&self) -> Self::Iter {
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

impl PerformWithClone for TicTacToe {
    fn perform(&mut self, m: &Self::Move) {
        self.place_unchecked(*m);
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use rival::{Evaluate, Moves, Play, SearchExt};
    use test::Bencher;

    use crate::TicTacToe;

    #[test]
    fn test_tictactoe() {
        let mut game = TicTacToe::new();

        for _ in 0..9 {
            let m = game.best_move(6).unwrap();
            game.play(&m);
        }

        assert_eq!(game.moves().len(), 0);
        assert_eq!(game.evaluate(), [0, 0]);
    }

    #[bench]
    fn bench_tictactoe(bencher: &mut Bencher) {
        bencher.iter(|| {
            let mut game = TicTacToe::new();

            for _ in 0..9 {
                let m = game.best_move(6).unwrap();
                game.play(&m);
            }
        });
    }
}
