use std::error::Error;
use std::fmt::{Display, Formatter};

pub type RivalResult<T, E = RivalError> = Result<T, E>;

#[derive(Debug)]
pub enum RivalError {
    NoMove,
}

impl Display for RivalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RivalError::NoMove => write!(f, "no move possible"),
        }
    }
}

impl Error for RivalError {}
