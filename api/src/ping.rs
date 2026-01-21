use regex::Regex;
use reqwest::StatusCode;

use super::{endpoints, ServerError};

// Return true if server available
pub async fn ping(http_client: reqwest::Client, addr: &str) -> Result<bool, ServerError> {
    let addr_regexp = Regex::new(r"[a-z0-9:.]+[:][1-9][0-9]+").unwrap();
    if addr_regexp.is_match(addr) {
        return Err(super::ServerError::new("address have bad syntax"))
    }

    let resp =  http_client.post(format!("https://{}{}{}", addr, endpoints::API_URL, endpoints::PING))
        .send()
        .await;

    match resp {
        Ok(resp) => match resp.status() {
            StatusCode::OK => Ok(true),
            _ => Ok(false)
        },
        Err(err) => { Err(ServerError::new(err.to_string().as_str())) }
    }
}

