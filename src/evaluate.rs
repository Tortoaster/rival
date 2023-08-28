pub type Value = i16;

/// Describes how the search algorithm should estimate which player is doing
/// best in the current state of the game. The const generic `N` describes the
/// number of players. The [`Evaluate2`] trait can be implemented instead for
/// 2-player games.
pub trait Evaluate<const N: usize> {
    /// Indicates whose turn it is. The values returned by [`evaluate`] should
    /// consistently return the score associated with the current player at this
    /// index.
    ///
    /// [`evaluate`]: Self::evaluate
    fn turn(&self) -> usize;

    /// Evaluates the current state, returning the respective scores for each
    /// player. Higher is better.
    fn evaluate(&self) -> [Value; N];

    /// Indicates whether the current state of the game is conclusive enough to
    /// stop searching here. If big changes are about to happen, such as a queen
    /// being under attack, this function may return false to instruct the
    /// search algorithm to continue.
    ///
    /// Returning `false` too often can dramatically slow down search.
    fn quiet(&self) -> bool {
        true
    }
}

/// A slightly simpler trait to implement than the more general [`Evaluate`]
/// trait, but only applicable for 2-player games.
pub trait Evaluate2 {
    /// Indicates whether the current playing is trying to minimize the score.
    /// This should be reflected by [`evaluate`] returning a lower score if this
    /// player is doing well.
    ///
    /// [`evaluate`]: Self::evaluate
    fn min_turn(&self) -> bool;

    /// Evaluates the current state, returning a high score if the maximizing
    /// player is winning, and a low score if the minimizing player is doing
    /// better.
    fn evaluate(&self) -> Value;

    /// Indicates whether the current state of the game is conclusive enough to
    /// stop searching here. If big changes are about to happen, such as a queen
    /// being under attack, this function may return false to instruct the
    /// search algorithm to continue.
    ///
    /// Returning `false` too often can dramatically slow down search.
    fn quiet(&self) -> bool {
        true
    }
}

impl<G: Evaluate2> Evaluate<2> for G {
    fn turn(&self) -> usize {
        self.min_turn() as usize
    }

    fn evaluate(&self) -> [Value; 2] {
        let value = self.evaluate();
        [value, -value]
    }

    fn quiet(&self) -> bool {
        self.quiet()
    }
}
