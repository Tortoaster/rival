pub trait Moves {
    type Move;
    type Iter<'a>: Iterator<Item = Self::Move>
    where
        Self: 'a;

    fn moves(&self) -> Self::Iter<'_>;
}
