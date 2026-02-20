//* ------------------------------- *//
//*                                 *//
//*   Mhserver API version: 1.2.x   *//
//*                                 *//
//* ------------------------------- *//

use regex::Regex;

use super::{endpoints, ServerError};

// Return true if server available
pub async fn ping(http_client: reqwest::Client, addr: &str) -> Result<(), ServerError> {
    let addr_regexp = Regex::new(r"[a-z0-9:.]+[:][1-9][0-9]+").unwrap();
    if !addr_regexp.is_match(addr) {
        return Err(super::ServerError::new("address have bad syntax"))
    }

    let resp =  http_client.post(endpoints::build_url(addr, endpoints::API_V1, endpoints::PING, None).unwrap())
        .send()
        .await;

    match resp {
        Ok(_) => Ok(()),
        Err(err) => { Err(ServerError::new(err.to_string().as_str())) }
    }
}

