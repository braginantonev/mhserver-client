use {
    super::{ States },
};

pub async fn connect(client: reqwest::Client, addr: &str, state: States) -> Result<States, String> {
    match api::ping::ping(client, addr).await {
        Ok(res) => {
            if res { Ok(state.next()) }
            else { Err("Wrong address or server is off".to_string()) }
        },
        Err(err) => { Err(format!("Connection error: {}", err.to_string())) }
    }
}