//! Rival is a framework for creating computer players for turn-based games.

#![doc(html_root_url = "https://docs.rs/rival/0.1.0")]
#![warn(missing_docs)]

pub mod cache;
pub mod game;
mod search;

// Definitions used in doctests, not public API
// TODO: Add #[cfg(doctest)] when https://github.com/rust-lang/rust/issues/67295 is fixed
#[doc(hidden)]
pub mod docs;
