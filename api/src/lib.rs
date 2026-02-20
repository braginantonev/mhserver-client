pub mod ping;
pub mod auth;
pub mod data;
mod endpoints;

use std::fmt;

#[derive(Debug)]
pub struct ServerError(String);

impl ServerError {
    pub fn to_string(&self) -> String {
        self.0.clone()
    }

    pub fn to_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&str> for ServerError {
    fn from(value: &str) -> Self {
        ServerError(value.to_owned())
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Server error: {}", self.0)
    }
}

impl std::error::Error for ServerError {}