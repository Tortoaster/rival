use std::{fmt, ops};

use adversary::search::{Game, WithCache};

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

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::X => write!(f, "X"),
            Symbol::O => write!(f, "O"),
        }
    }
}

#[derive(Clone)]
struct Place {
    x: usize,
    y: usize,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct TicTacToe {
    turn: Symbol,
    field: [[Option<Symbol>; 3]; 3],
}

impl Game<2> for TicTacToe {
    type Move = Place;

    const DEPTH: u32 = 9;

    fn turn(&self) -> usize {
        self.turn as usize
    }

    fn evaluate(&self) -> [i32; 2] {
        let triplets = vec![
            ((0, 0), (0, 1), (0, 2)),
            ((1, 0), (1, 1), (1, 2)),
            ((2, 0), (2, 1), (2, 2)),
            ((0, 0), (1, 0), (2, 0)),
            ((0, 1), (1, 1), (2, 1)),
            ((0, 2), (1, 2), (2, 2)),
            ((0, 0), (1, 1), (2, 2)),
            ((0, 2), (1, 1), (2, 0))
        ];
        let winners: Vec<Symbol> = triplets
            .into_iter()
            .flat_map(|((x1, y1), (x2, y2), (x3, y3))|
                (self.field[x1][y1].is_some() && self.field[x1][y1] == self.field[x2][y2] && self.field[x2][y2] == self.field[x3][y3])
                    .then(|| self.field[x1][y1].unwrap())
            )
            .collect();
        if winners.is_empty() {
            [0, 0]
        } else {
            match winners[0] {
                Symbol::X => [1, -1],
                Symbol::O => [-1, 1]
            }
        }
    }

    fn get_moves(&self) -> Vec<Place> {
        if self.evaluate()[0] != 0 {
            Vec::new()
        } else {
            self.field
                .iter()
                .enumerate()
                .flat_map(|(x, row)| row
                    .iter()
                    .enumerate()
                    .flat_map(move |(y, symbol)| symbol.is_none().then(|| Place { x, y }))
                )
                .collect()
        }
    }

    fn perform(&mut self, action: &Place) {
        if self.field[action.x][action.y].replace(self.turn).take().is_some() {
            panic!("There is already a symbol here")
        };
        self.turn = !self.turn;
    }

    fn undo(&mut self, action: &Place) {
        self.turn = !self.turn;
        self.field[action.x][action.y].take().expect("There is no symbol here");
    }
}

impl fmt::Display for TicTacToe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, " {} | {} | {} \n---+---+---\n {} | {} | {} \n---+---+---\n {} | {} | {} ",
                 self.field[0][0].map(|s| format!("{}", s)).unwrap_or(" ".to_owned()),
                 self.field[1][0].map(|s| format!("{}", s)).unwrap_or(" ".to_owned()),
                 self.field[2][0].map(|s| format!("{}", s)).unwrap_or(" ".to_owned()),
                 self.field[0][1].map(|s| format!("{}", s)).unwrap_or(" ".to_owned()),
                 self.field[1][1].map(|s| format!("{}", s)).unwrap_or(" ".to_owned()),
                 self.field[2][1].map(|s| format!("{}", s)).unwrap_or(" ".to_owned()),
                 self.field[0][2].map(|s| format!("{}", s)).unwrap_or(" ".to_owned()),
                 self.field[1][2].map(|s| format!("{}", s)).unwrap_or(" ".to_owned()),
                 self.field[2][2].map(|s| format!("{}", s)).unwrap_or(" ".to_owned()),
        )
    }
}

#[test]
fn tictactoe() {
    let mut game = TicTacToe {
        turn: Symbol::O,
        field: [[None; 3]; 3],
    };

    for _ in 0..9 {
        let m = game.find_best().unwrap();
        game.perform(&m);
        println!("{}", game);
    }

    assert!(game.get_moves().is_empty());
    assert_eq!(game.evaluate(), [0, 0]);
}

#[test]
fn cached_tictactoe() {
    let mut game = TicTacToe {
        turn: Symbol::O,
        field: [[None; 3]; 3],
    }.with_cache(20000);

    for _ in 0..9 {
        let m = game.find_best().unwrap();
        game.perform(&m);
        println!("{}", game);
    }

    assert!(game.get_moves().is_empty());
    assert_eq!(game.evaluate(), [0, 0]);
}
