use crate::moves::Moves;

pub trait Perform: Moves {
    type Remember;

    fn perform(&mut self, m: &Self::Move) -> Self::Remember;

    fn revert(&mut self, remember: Self::Remember);
}

pub trait SimplePerform: Moves {
    fn perform(&mut self, m: &Self::Move);
}

impl<G> Perform for G
where
    G: SimplePerform + Moves + Clone,
{
    type Remember = Self;

    fn perform(&mut self, m: &Self::Move) -> Self::Remember {
        let clone = self.clone();
        self.perform(m);
        clone
    }

    fn revert(&mut self, remember: Self::Remember) {
        *self = remember;
    }
}
