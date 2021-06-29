use rival::cache::WithCache;
use rival::game::Game;
use rival::games::TicTacToe;

#[test]
fn tictactoe() {
    let mut game = TicTacToe::new();

    for _ in 0..9 {
        let m = game.best_move().unwrap();
        game.perform(&m);
        println!("{}", game);
    }

    assert!(game.moves().is_empty());
    assert_eq!(game.value(), [0, 0]);
}

#[test]
fn cached_tictactoe() {
    let mut game = TicTacToe::new().with_cache(20000);

    for _ in 0..9 {
        let m = game.best_move().unwrap();
        game.perform(&m);
        println!("{}", game);
    }

    assert!(game.moves().is_empty());
    assert_eq!(game.value(), [0, 0]);
}
