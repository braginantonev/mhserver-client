pub struct AuthServiceConfig {
    server_address: String,
}

impl AuthServiceConfig {
    pub fn new(srv_addr: &str) -> Self {
        Self { 
            server_address: srv_addr.to_owned(),
        }
    }

    pub fn server_address(&self) -> Option<&str> {
        if self.server_address != "" {
            Some(self.server_address.as_str())
        } else {
            None
        }
    }

    pub fn set_server_address(&mut self, new_addr: &str) {
        self.server_address = new_addr.to_owned()
    }
}
