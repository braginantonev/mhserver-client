use std::fmt;

pub mod ping;
pub mod auth;
mod endpoints;

#[derive(Debug)]
pub struct ServerError(String);

impl ServerError {
    pub fn new(description: &str) -> Self {
        ServerError(description.to_string())
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }

    pub fn to_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Server error: {}", self.0)
    }
}

impl std::error::Error for ServerError {}