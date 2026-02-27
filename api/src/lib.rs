pub mod ping;
pub mod auth;
pub mod data;
mod endpoints;

use std::fmt;

use reqwest::{StatusCode, Client};

pub struct AuthorizedRequest {
    client: Client,
    srv_addr: String,
    jwt: String
}

impl AuthorizedRequest {
    pub fn new(client: Client, srv_addr: &str, jwt: &str) -> Self {
        Self { client, srv_addr: srv_addr.to_owned(), jwt: jwt.to_owned() }
    }

    pub fn srv_addr_str(&self) -> &str {
        self.srv_addr.as_str()
    }
}

#[derive(Debug)]
pub struct ServerError {
    desc: String,
    status: Option<StatusCode>
}

impl ServerError {
    pub fn new(desc: &str) -> Self {
        Self { desc: desc.to_owned(), status: None }
    }

    pub fn with_status(mut self, st: StatusCode) -> Self {
        self.status = Some(st);
        self
    }
    
    pub fn desc(&self) -> String {
        self.desc.clone()
    }

    pub fn status(&self) -> Option<StatusCode> {
        self.status
    }
}

impl From<&str> for ServerError {
    fn from(value: &str) -> Self {
        Self { desc: value.to_owned(), status: Some(StatusCode::BAD_REQUEST) }
    }
}

impl From<reqwest::Error> for ServerError {
    fn from(value: reqwest::Error) -> Self {
        Self { desc: value.to_string(), status: value.status() }
    }
}

impl From<url::ParseError> for ServerError {
    fn from(value: url::ParseError) -> Self {
        Self { desc: value.to_string(), status: Some(StatusCode::BAD_REQUEST) }
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let st_code = match self.status {
            Some(st) => st.as_u16(),
            None => 0
        };

        write!(f, "Server error ({}): {}", st_code, self.desc)
    }
}

impl std::error::Error for ServerError {}