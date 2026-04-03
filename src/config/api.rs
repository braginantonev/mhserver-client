use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ServerApiConfiguration {
    base_path: String,
    jwt: String
}

impl ServerApiConfiguration {
    pub fn new(base_path: String, jwt: String) -> Self {
        Self { base_path, jwt }
    }

    pub fn base_path(&self) -> &str {
        self.base_path.as_str()
    }

    pub fn jwt(&self) -> &str {
        self.jwt.as_str()
    }

    pub fn set_base_path(&mut self, path: &str) {
        self.base_path = path.to_owned();
    }

    pub fn set_jwt(&mut self, token: &str) {
        self.jwt = token.to_owned();
    }
}