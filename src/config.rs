use {
    serde::{ Serialize, Deserialize },
    std::{env, fs},
};

#[cfg(target_os = "linux")]
const CONFIG_PATH: &str = "/.config/mhserver-client";

#[cfg(target_os = "windows")]
const CONFIG_PATH: &str = "\\AppData\\Local\\mhserver-client"; 

fn config_path() -> String {
    format!("{}/{}", env::home_dir().expect("failed get user home dir").display(), CONFIG_PATH)
}

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(rename = "ServerAddress")]
    server_address: String,

    #[serde(rename = "UserJWT")]
    user_jwt: String,
}

impl Config {
    pub fn from_file() -> Result<Self, std::io::Error> {
        match fs::read_to_string(config_path()) {
            Ok(read) => Ok(toml::from_str::<Config>(read.as_str()).expect("failed convert toml file to Config")),
            Err(err) => Err(err)
        }
    }

    pub fn get_server_address(&self) -> String {
        self.server_address.clone()
    }

    pub fn get_user_jwt(&self) -> String {
        self.user_jwt.clone()
    }

    pub fn set_address(&mut self, new_addr: &str) {
        self.server_address = new_addr.to_string()
    }

    pub fn set_user_jwt(&mut self, new_jwt: &str) {
        self.user_jwt = new_jwt.to_string();
    }
}


