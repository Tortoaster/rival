pub trait Moves {
    type Move;
    type Iter: Iterator<Item = Self::Move>;

    fn moves(&self) -> Self::Iter;
}
