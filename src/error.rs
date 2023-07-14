use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Error {
    pub(crate) message: String
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error { message: value.to_string() }
    }
}