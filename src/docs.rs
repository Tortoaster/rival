use std::{fmt, ops};

use crate::game::Game;

#[doc = include_str!("../README.md")]
pub struct ReadMe;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TicTacToe {
    turn: Symbol,
    field: [[Option<Symbol>; 3]; 3],
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            turn: Symbol::O,
            field: [[None; 3]; 3],
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Symbol {
    X = 0,
    O = 1,
}

impl ops::Not for Symbol {
    type Output = Symbol;

    fn not(self) -> Self::Output {
        match self {
            Symbol::X => Symbol::O,
            Symbol::O => Symbol::X,
        }
    }
}

#[derive(Clone)]
pub struct Place {
    x: usize,
    y: usize,
}

impl Game<2> for TicTacToe {
    type Move = Place;

    const DEPTH: u32 = 9;

    fn turn(&self) -> usize {
        self.turn as usize
    }

    fn value(&self) -> [i32; 2] {
        let triplets = vec![
            ((0, 0), (0, 1), (0, 2)),
            ((1, 0), (1, 1), (1, 2)),
            ((2, 0), (2, 1), (2, 2)),
            ((0, 0), (1, 0), (2, 0)),
            ((0, 1), (1, 1), (2, 1)),
            ((0, 2), (1, 2), (2, 2)),
            ((0, 0), (1, 1), (2, 2)),
            ((0, 2), (1, 1), (2, 0)),
        ];
        let winners: Vec<Symbol> = triplets
            .into_iter()
            .flat_map(|((x1, y1), (x2, y2), (x3, y3))| {
                (self.field[x1][y1].is_some()
                    && self.field[x1][y1] == self.field[x2][y2]
                    && self.field[x2][y2] == self.field[x3][y3])
                    .then(|| self.field[x1][y1].unwrap())
            })
            .collect();
        if winners.is_empty() {
            [0, 0]
        } else {
            match winners[0] {
                Symbol::X => [1, -1],
                Symbol::O => [-1, 1],
            }
        }
    }

    fn moves(&self) -> Vec<Place> {
        if self.value()[0] != 0 {
            Vec::new()
        } else {
            self.field
                .iter()
                .enumerate()
                .flat_map(|(x, row)| {
                    row.iter()
                        .enumerate()
                        .flat_map(move |(y, symbol)| symbol.is_none().then(|| Place { x, y }))
                })
                .collect()
        }
    }

    fn perform(&mut self, action: &Place) {
        if self.field[action.x][action.y]
            .replace(self.turn)
            .take()
            .is_some()
        {
            panic!("There is already a symbol here")
        };
        self.turn = !self.turn;
    }

    fn revert(&mut self, action: &Place) {
        self.turn = !self.turn;
        self.field[action.x][action.y]
            .take()
            .expect("There is no symbol here");
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::X => write!(f, "X"),
            Symbol::O => write!(f, "O"),
        }
    }
}

impl fmt::Display for TicTacToe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            " {} | {} | {} \n---+---+---\n {} | {} | {} \n---+---+---\n {} | {} | {} ",
            self.field[0][0]
                .map(|s| format!("{}", s))
                .unwrap_or(" ".to_owned()),
            self.field[1][0]
                .map(|s| format!("{}", s))
                .unwrap_or(" ".to_owned()),
            self.field[2][0]
                .map(|s| format!("{}", s))
                .unwrap_or(" ".to_owned()),
            self.field[0][1]
                .map(|s| format!("{}", s))
                .unwrap_or(" ".to_owned()),
            self.field[1][1]
                .map(|s| format!("{}", s))
                .unwrap_or(" ".to_owned()),
            self.field[2][1]
                .map(|s| format!("{}", s))
                .unwrap_or(" ".to_owned()),
            self.field[0][2]
                .map(|s| format!("{}", s))
                .unwrap_or(" ".to_owned()),
            self.field[1][2]
                .map(|s| format!("{}", s))
                .unwrap_or(" ".to_owned()),
            self.field[2][2]
                .map(|s| format!("{}", s))
                .unwrap_or(" ".to_owned()),
        )
    }
}
