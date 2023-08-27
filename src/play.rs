use crate::moves::Moves;

pub trait Play: Moves {
    type Remember;

    fn play(&mut self, m: &Self::Move) -> Self::Remember;

    fn unplay(&mut self, remember: Self::Remember);
}

pub trait PerformWithClone: Moves {
    fn perform(&mut self, m: &Self::Move);
}

impl<G> Play for G
where
    G: PerformWithClone + Moves + Clone,
{
    type Remember = Self;

    fn play(&mut self, m: &Self::Move) -> Self::Remember {
        let clone = self.clone();
        self.perform(m);
        clone
    }

    fn unplay(&mut self, remember: Self::Remember) {
        *self = remember;
    }
}
