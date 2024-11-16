#![feature(test)]

use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Index, IndexMut, Not},
};

use rival::{CloneCacheKey, EvaluateZeroSum, LazyZobristHash, Moves, PlayClone, Value};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Symbol {
    X,
    O,
}

impl Symbol {
    fn value(self) -> i16 {
        match self {
            Symbol::X => -1,
            Symbol::O => 1,
        }
    }
}

impl Not for Symbol {
    type Output = Symbol;

    fn not(self) -> Self::Output {
        match self {
            Symbol::X => Symbol::O,
            Symbol::O => Symbol::X,
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::X => write!(f, "X"),
            Symbol::O => write!(f, "O"),
        }
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct TicTacToe {
    turn: Symbol,
    grid: [[Option<Symbol>; 3]; 3],
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            turn: Symbol::O,
            grid: [[None; 3]; 3],
        }
    }
}

impl Index<(usize, usize)> for TicTacToe {
    type Output = Option<Symbol>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[index.0][index.1]
    }
}

impl IndexMut<(usize, usize)> for TicTacToe {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.grid[index.0][index.1]
    }
}

impl EvaluateZeroSum for TicTacToe {
    fn min_turn(&self) -> bool {
        self.turn == Symbol::X
    }

    fn evaluate(&self) -> Value {
        let triplets = [
            // Rows
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            // Columns
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            // Diagonals
            [(0, 0), (1, 1), (2, 2)],
            [(2, 0), (1, 1), (0, 2)],
        ];

        triplets
            .iter()
            .filter(|triplet| {
                self[triplet[0]] == self[triplet[1]] && self[triplet[1]] == self[triplet[2]]
            })
            .find_map(|triplet| self[triplet[0]])
            .map(|symbol| symbol.value())
            .unwrap_or(0)
    }
}

impl Moves for TicTacToe {
    type Move = (usize, usize);
    type Iter<'a> = <Vec<Self::Move> as IntoIterator>::IntoIter;

    fn moves(&self) -> Self::Iter<'static> {
        if self.evaluate() != 0 {
            return Vec::new().into_iter();
        }

        let moves: Vec<_> = self
            .grid
            .iter()
            .enumerate()
            .flat_map(|(x, column)| {
                column.iter().enumerate().filter_map(move |(y, symbol)| {
                    if symbol.is_none() {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .collect();

        moves.into_iter()
    }
}

impl PlayClone for TicTacToe {
    fn play(&mut self, m: &Self::Move) {
        self[*m] = Some(self.turn);
        self.turn = !self.turn;
    }
}

impl LazyZobristHash for TicTacToe {}

impl CloneCacheKey for TicTacToe {}

impl Display for TicTacToe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}|{}|{}\n-+-+-\n{}|{}|{}\n-+-+-\n{}|{}|{}",
            self[(0, 0)]
                .map(|symbol| symbol.to_string())
                .unwrap_or(" ".to_owned()),
            self[(1, 0)]
                .map(|symbol| symbol.to_string())
                .unwrap_or(" ".to_owned()),
            self[(2, 0)]
                .map(|symbol| symbol.to_string())
                .unwrap_or(" ".to_owned()),
            self[(0, 1)]
                .map(|symbol| symbol.to_string())
                .unwrap_or(" ".to_owned()),
            self[(1, 1)]
                .map(|symbol| symbol.to_string())
                .unwrap_or(" ".to_owned()),
            self[(2, 1)]
                .map(|symbol| symbol.to_string())
                .unwrap_or(" ".to_owned()),
            self[(0, 2)]
                .map(|symbol| symbol.to_string())
                .unwrap_or(" ".to_owned()),
            self[(1, 2)]
                .map(|symbol| symbol.to_string())
                .unwrap_or(" ".to_owned()),
            self[(2, 2)]
                .map(|symbol| symbol.to_string())
                .unwrap_or(" ".to_owned())
        )
    }
}

impl Debug for TicTacToe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use rival::{EvaluateZeroSum, Moves, Negamax, PlayClone, Rival};
    use test::Bencher;

    use crate::TicTacToe;

    /// Capacity of the transposition table of computer players in these tests.
    const CAP: usize = 2000;

    // #[test]
    // fn test_tictactoe_maxn_tie() {
    //     let mut game = TicTacToe::new();
    //     let mut rival: Rival<_, MaxN, 2, CAP> = Rival::new();
    //
    //     for _ in 0..9 {
    //         assert_eq!(rival.play(&mut game, 9), Ok(()), "{game}");
    //     }
    //
    //     assert_eq!(game.moves().len(), 0);
    //     assert_eq!(game.evaluate(), 0);
    // }

    // #[test]
    // fn test_tictactoe_maxn_best_move() {
    //     let mut game = TicTacToe::new();
    //
    //     game.play(&(0, 0));
    //     game.play(&(1, 0));
    //     game.play(&(0, 1));
    //
    //     let mut rival: Rival<_, MaxN, 2, CAP> = Rival::new();
    //     assert_eq!(rival.play(&mut game, 9), Ok(()), "{game}");
    //
    //     assert_ne!(game[(0, 2)], None);
    // }

    #[test]
    fn test_tictactoe_negamax_tie() {
        let mut game = TicTacToe::new();
        let mut rival: Rival<_, Negamax, 2, CAP> = Rival::new();

        for _ in 0..9 {
            assert_eq!(rival.play(&mut game, 9), Ok(()), "{game}");
        }

        assert_eq!(game.moves().len(), 0);
        assert_eq!(game.evaluate(), 0);
    }

    #[test]
    fn test_tictactoe_negamax_best_move() {
        let mut game = TicTacToe::new();

        game.play(&(0, 0));
        game.play(&(1, 0));
        game.play(&(0, 1));

        let mut rival: Rival<_, Negamax, 2, CAP> = Rival::new();
        assert_eq!(rival.play(&mut game, 9), Ok(()), "{game}");

        assert_ne!(game[(0, 2)], None);
    }

    // #[test]
    // fn test_tictactoe_maxn_vs_negamax_tie() {
    //     let mut game = TicTacToe::new();
    //     let mut a: Rival<_, Negamax, 2, CAP> = Rival::new();
    //     let mut b: Rival<_, MaxN, 2, CAP> = Rival::new();
    //
    //     for _ in 0..4 {
    //         assert_eq!(a.play(&mut game, 9), Ok(()), "{game}");
    //         assert_eq!(b.play(&mut game, 9), Ok(()), "{game}");
    //     }
    //     assert_eq!(a.play(&mut game, 9), Ok(()), "{game}");
    //
    //     assert_eq!(game.moves().len(), 0);
    //     assert_eq!(game.evaluate(), 0);
    // }

    // #[bench]
    // fn bench_tictactoe_maxn(bencher: &mut Bencher) {
    //     let mut rival: Rival<_, MaxN, 2, CAP> = Rival::new();
    //
    //     bencher.iter(|| {
    //         let mut game = TicTacToe::new();
    //
    //         for _ in 0..9 {
    //             rival.play(&mut game, 9).unwrap();
    //         }
    //     });
    // }

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
