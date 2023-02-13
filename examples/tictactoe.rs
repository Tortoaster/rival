#![feature(test)]

#[cfg(test)]
mod tests {
    extern crate test;

    use test::Bencher;

    use rival::cache::WithCache;
    use rival::docs::TicTacToe;
    use rival::game::Game;

    #[test]
    fn test_tictactoe() {
        let mut game = TicTacToe::new();

        for _ in 0..9 {
            let m = game.best_move().unwrap();
            game.perform(&m);
            println!("{}", game);
        }

        assert!(game.moves().is_empty());
        assert_eq!(game.value(), [0, 0]);
    }

    #[bench]
    fn bench_tictactoe(bencher: &mut Bencher) {
        bencher.iter(|| {
            let mut game = TicTacToe::new();

            for _ in 0..9 {
                let m = game.best_move().unwrap();
                game.perform(&m);
            }
        });
    }

    #[bench]
    fn bench_tictactoe_cached(bencher: &mut Bencher) {
        bencher.iter(|| {
            let mut game = TicTacToe::new().with_cache(20000);

            for _ in 0..9 {
                let m = game.best_move().unwrap();
                game.perform(&m);
            }
        });
    }
}
