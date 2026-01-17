use {
    super::{ AppStates },
    reqwest::Client
};

pub struct ServerConnection {
    client: Client,
    addr: String,
}

impl ServerConnection {
    pub fn new(client: Client, addr: &str) -> Self {
        Self { client: client, addr: addr.to_string() }
    }

    pub fn set_address(&mut self, new_addr: &str) {
        self.addr = new_addr.to_string()
    }

    pub fn get_address(&self) -> String {
        self.addr.clone()
    }

    // Return cloned client
    pub fn get_client(&self) -> Client {
        self.client.clone()
    }
}

pub async fn connect(client: Client, addr: &str, state: AppStates) -> Result<AppStates, String> {
    match api::ping::ping(client, addr).await {
        Ok(res) => {
            if res { Ok(state.next()) }
            else { Err("Wrong address or server is off".to_string()) }
        },
        Err(err) => { Err(format!("Connection error: {}", err.to_string())) }
    }
}