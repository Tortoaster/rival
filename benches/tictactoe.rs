#![feature(test)]
extern crate test;

use test::Bencher;

use rival::cache::WithCache;
use rival::game::Game;
use rival::games::TicTacToe;

#[bench]
fn tictactoe(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut game = TicTacToe::new();

        for _ in 0..9 {
            let m = game.best_move().unwrap();
            game.perform(&m);
        }
    });
}

#[bench]
fn cached_tictactoe(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut game = TicTacToe::new().with_cache(20000);

        for _ in 0..9 {
            let m = game.best_move().unwrap();
            game.perform(&m);
        }
    });
}
