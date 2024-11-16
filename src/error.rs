use std::{
    error::Error,
    fmt::{Display, Formatter},
};

pub type RivalResult<T, E = RivalError> = Result<T, E>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
