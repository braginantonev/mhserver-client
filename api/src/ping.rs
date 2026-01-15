use std::net::ToSocketAddrs;

use regex::Regex;

fn addr_formatted(addr: &str) -> bool {
    let re = Regex::new(r"[a-z0-9:.]+[:][1-9][0-9]+").unwrap();
    re.is_match(addr)
}

// Return true if server available
pub fn ping(addr: &str) -> Result<bool, super::ServerError> {
    if !addr_formatted(addr) {
        return Err(super::ServerError::new("address have bad syntax"))
    }

    let ip = addr
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap()
        .ip();

    match ping::new(ip).send() {
        Ok(_) => return Ok(true),
        Err(_) => Ok(false),
    }
}

