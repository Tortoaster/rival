use crate::moves::Moves;

/// Describes how the game state should change when playing a move, and how a
/// move can be reverted. The latter is necessary for the search algorithm. As
/// an alternative, the [`PlayClone`] trait is safe to implement, and only
/// requires the implementation of [`play`].
///
/// # Safety
///
/// Calling [`play`] on a state with a certain move, and then calling [`unplay`]
/// with the returned [`Remember`], *must* result in all references to the
/// initial state to still point to valid memory.
///
/// This trait is unsafe because the [`Moves`] trait requires a lifetime on the
/// returned iterator to be usable in practice. This interferes with the search
/// algorithm, as it needs to play multiple consecutive moves, which requires
/// mutating state while also borrowing it to generate alternative moves later.
/// As long as the implementation of this trait can undo moves without
/// invalidating existing references, the search algorithm can assume that
/// holding the mutable and immutable borrow at the same time is safe.
///
/// [`play`]: Self::play
/// [`unplay`]: Self::unplay
/// [`Remember`]: Self::Remember
pub unsafe trait Play: Moves {
    /// Type of the intermediate data that allows a [`play`]ed move to be
    /// [`unplay`]ed.
    ///
    /// [`play`]: Self::play
    /// [`unplay`]: Self::unplay
    type Remember;

    /// Describes how the state should change in response to playing a move. May
    /// return data of type [`Remember`] to help [`unplay`]ing the move later.
    ///
    /// [`unplay`]: Self::unplay
    /// [`Remember`]: Self::Remember
    fn play(&mut self, m: &Self::Move) -> Self::Remember;

    /// Describes how to undo playing a move, reverting to the original state.
    /// Keep in mind that any references to the state prior to playing the move
    /// should be valid after unplaying.
    fn unplay(&mut self, remember: Self::Remember);
}

/// A safe and simple alternative to the [`Play`] trait. This implementation
/// simply clones the original state before playing a move, and puts it back to
/// [`unplay`]. In most cases, this trait is more than adequate, but some games
/// might gain a performance boost from implementing [`Play`] directly.
///
/// [`unplay`]: Play::unplay
pub trait PlayClone: Moves {
    fn play(&mut self, m: &Self::Move);
}

unsafe impl<G> Play for G
where
    G: PlayClone + Moves + Clone,
{
    type Remember = Self;

    fn play(&mut self, m: &Self::Move) -> Self::Remember {
        let clone = self.clone();
        self.play(m);
        clone
    }

    fn unplay(&mut self, remember: Self::Remember) {
        *self = remember;
    }
}
