use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug, Clone)]
pub enum KitsuError {
    AnimeNotFound,
}

impl Error for KitsuError {}

impl Display for KitsuError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            KitsuError::AnimeNotFound => write!(f, "Anime does not exist"),
        }
    }
}
