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
    match client.get(build_url(addr, API_V1, LOGIN, None).unwrap())
        .json(&user)
        .send()
        .await {
            Ok(resp) => {
                if resp.status() != 200 {
                    let st = resp.status();
                    return Err(ServerError::new(resp.text().await.unwrap().as_str(), st)) 
                }
                else { return Ok(resp.text().await.expect("failed get response text")) }
            }
            Err(err) => { return Err(ServerError::from(err)) }
        };
}

pub async fn register_v1(client: Client, addr: &str, user: User) -> Result<(), ServerError> {
    match client.post(build_url(addr, API_V1, REGISTRATION, None).unwrap())
        .json(&user)
        .send()
        .await {
            Ok(resp) => {
                if resp.status() != 200 { 
                    let st = resp.status();
                    return Err(ServerError::new(resp.text().await.unwrap().as_str(), st)) 
                }
            }
            Err(err) => { return Err(ServerError::from(err)) }
        };
    
    Ok(())
}