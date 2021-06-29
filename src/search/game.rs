use crate::search::{Search, Value};

/// This trait can be implemented on any type that represents the state of a game. Doing so requires
/// implementations that inform the trait of which moves each player can [`Game::perform`] and how
/// well each player is doing in any particular state. In return, this trait will provide a method
/// that calculates the [`Game::best_move`] the current player can make.
///
/// The const generic in [`Game<N>`] represents the number of players in this game.
pub trait Game<const N: usize> {
    /// The type of the moves that players can [`Game::perform`].
    type Move;

    /// The number of moves to look ahead when searching the [`Game::best_move`].
    const DEPTH: u32;

    /// Returns the index of the player whose turn it is.
    fn turn(&self) -> usize;

    /// Evaluates the game's state and returns the relative value for each of the players,
    /// corresponding to their index according to [`Game::turn`].
    ///
    /// For two-player games, the sum of the values is usually 0: if the second player of a game of
    /// tic-tac-toe is the winner, the result of this method could be \[-1, 1\].
    fn value(&self) -> [Value; N];

    /// Returns a [`Vec`] of moves that the current player (see [`Game::turn`]) can
    /// [`Game::perform`].
    fn moves(&self) -> Vec<Self::Move>;

    /// Performs the given [`Game::Move`], changing the game's state.
    fn perform(&mut self, m: &Self::Move);

    /// Reverts the given [`Game::Move`]. This is the dual of [`Game::perform`]: [`Game::revert`]
    /// after [`Game::perform`] with identical moves should have no effect on game's state.
    fn revert(&mut self, m: &Self::Move);

    /// Returns the best [`Game::Move`] for the current player (see [`Game::turn`]), according to
    /// the calculated [`Game::value`] within the next [`Game::DEPTH`] moves, or [`None`] if the
    /// current player cannot perform any move.
    fn best_move(&mut self) -> Option<Self::Move> {
        self.max_n(Self::DEPTH, &mut [Value::MIN; N]).best
    }

    /// Indicates whether the current evaluation of the game state will not change much within
    /// subsequent moves.
    ///
    /// If the balance of the game is likely to shift drastically within the next move -- such as
    /// when a queen in chess is under attack -- this method should return `false`, which will
    /// trigger the algorithm to continue searching even if the search [`Game::DEPTH`] is exceeded.
    /// This can improve the performance of the computer player, but it is not necessary to
    /// implement this method.
    ///
    /// # Termination
    ///
    /// Keep in mind that searching the [`Game::best_move`] might take forever if this function
    /// returns `false` too often.
    fn quiet(&self) -> bool {
        true
    }
}
