//* ------------------------------- *//
//*                                 *//
//*   Mhserver API version: 1.2.x   *//
//*                                 *//
//* ------------------------------- *//

use {
    super::{
        ServerError,
        endpoints::{ 
            build_url, API_V1, 
            auth::{ LOGIN, REGISTRATION }
        },
    },
    reqwest::Client,
    serde::{ Deserialize, Serialize },
};

#[derive(Serialize, Deserialize)]
pub struct User {
    user: String,
    pass: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> Self {
        Self { user: username.to_string(), pass: password.to_string() }
    }
}

/// Return response body
pub async fn login_v1(client: Client, addr: &str, user: User) -> Result<String, ServerError> {
    match client.get(build_url(addr, API_V1, LOGIN))
        .json(&user)
        .send()
        .await {
            Ok(resp) => {
                if resp.status() != 200 { return Err(ServerError(resp.text().await.expect("failed get response text"))) }
                else { return Ok(resp.text().await.expect("failed get response text")) }
            }
            Err(err) => { return Err(ServerError(err.to_string())) }
        };
}

pub async fn register_v1(client: Client, addr: &str, user: User) -> Result<(), ServerError> {
    match client.post(build_url(addr, API_V1, REGISTRATION))
        .json(&user)
        .send()
        .await {
            Ok(resp) => {
                if resp.status() != 200 { return Err(ServerError(resp.text().await.expect("failed get response text"))) }
            }
            Err(err) => { return Err(ServerError(err.to_string())) }
        };
    
    Ok(())
}