//! Custom errors.
//!
//! A small module that contains a custom error struct used to encapsulate errors from lower down in
//! the function call chain.
//!


use std::fmt::{Display, Formatter};

/// A custom error struct. Contains the string representation for the encapsulated error.
#[derive(Debug, Clone)]
pub struct Error {
    pub message: String
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