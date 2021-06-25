pub use cached_game::{Cached, WithCache};
pub use game::Game;

mod game;
mod cached_game;

pub type Value = i32;

#[derive(Clone, Debug)]
pub struct SearchResult<M, const N: usize> {
    depth: u32,
    value: [Value; N],
    best: Option<M>,
}
