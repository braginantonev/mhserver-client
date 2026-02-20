pub mod ping;
pub mod auth;
pub mod data;
mod endpoints;

use std::fmt;

use reqwest::StatusCode;

#[derive(Debug)]
pub struct ServerError {
    desc: String,
    status: StatusCode
}

impl ServerError {
    pub fn new(desc: &str, status: StatusCode) -> Self {
        Self { desc: desc.to_owned(), status }
    }
    
    pub fn desc(&self) -> String {
        self.desc.clone()
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }
}

impl From<&str> for ServerError {
    fn from(value: &str) -> Self {
        Self { desc: value.to_owned(), status: StatusCode::BAD_REQUEST }
    }
}

impl From<reqwest::Error> for ServerError {
    fn from(value: reqwest::Error) -> Self {
        Self { desc: value.to_string(), status: value.status().unwrap() }
    }
}

impl From<url::ParseError> for ServerError {
    fn from(value: url::ParseError) -> Self {
        Self { desc: value.to_string(), status: StatusCode::BAD_REQUEST }
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Server error (code: {}): {}", self.status, self.desc)
    }
}

impl std::error::Error for ServerError {}