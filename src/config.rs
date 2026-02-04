use {
    serde::{ Deserialize, Serialize },
    std::{env, fs, io::Error, path::PathBuf},
};

const CONFIG_PATH: &str = ".config/mhserver-client";
const CONFIG_FILENAME: &str = "general.conf";

#[derive(Serialize, Deserialize, Default)]
struct ServerCommunication {
    server_address: String,
    user_jwt: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(rename = "server_communication")]
    srv_com: ServerCommunication,
}

fn config_dir() -> PathBuf {
    env::home_dir().expect("failed get home dir").join(CONFIG_PATH)
}

fn config_file() -> PathBuf {
    config_dir().join(CONFIG_FILENAME)
}

impl Config {
    pub fn from_file() -> Result<Self, Error> {
        match fs::read_to_string(config_file()) {
            Ok(read) => Ok(toml::from_str(read.as_str()).expect("failed convert toml file to Config")),
            Err(err) => Err(err)
        }
    }

    pub fn save_to_file(&self) {
        if !fs::exists(config_file()).unwrap() {
            let _ = fs::create_dir_all(config_dir());
        }

        let _ = fs::write(
            config_file(),
            toml::to_string_pretty(&self).expect("failed serialize config to string")
        );
    }

    //* Server communication 

    pub fn get_server_address(&self) -> String {
        self.srv_com.server_address.clone()
    }

    pub fn get_user_jwt(&self) -> String {
        self.srv_com.user_jwt.clone()
    }

    pub fn set_address(&mut self, new_addr: &str) {
        self.srv_com.server_address = new_addr.to_string()
    }

    pub fn set_user_jwt(&mut self, new_jwt: &str) {
        self.srv_com.user_jwt = new_jwt.to_string();
    }
}


