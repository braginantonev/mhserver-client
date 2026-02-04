use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Default)]
pub struct ServerComConfig {
    server_address: String,
    user_jwt: String,
}

impl ServerComConfig {
    pub fn new(srv_addr: &str, user_jwt: &str) -> Self {
        Self { server_address: srv_addr.to_owned(), user_jwt: user_jwt.to_owned() }
    }

    pub fn server_address(&self) -> String {
        self.server_address.clone()
    }

    pub fn user_jwt(&self) -> String {
        self.user_jwt.clone()
    }

    pub fn set_user_jwt(&mut self, new_user_jwt: &str) {
        self.user_jwt = new_user_jwt.to_owned()
    }

    pub fn set_server_address(&mut self, new_srv_addr: &str) {
        self.server_address = new_srv_addr.to_owned()
    }
}