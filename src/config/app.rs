use {
    serde::{ Deserialize, Serialize },
    std::{env, fs, path::PathBuf},
    
    super::com::ServerComConfig,
};

const CONFIG_PATH: &str = ".config/mhserver-client";
const CONFIG_FILENAME: &str = "general.conf";

fn config_dir() -> PathBuf {
    env::home_dir().expect("failed get home dir").join(CONFIG_PATH)
}

fn config_file() -> PathBuf {
    config_dir().join(CONFIG_FILENAME)
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ApplicationConfig {
    #[serde(rename = "server_communication")]
    srv_com: ServerComConfig,
}

impl ApplicationConfig {
    pub fn from_file() -> Result<Self, std::io::Error> {
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

    pub fn server_com_config(&self) -> &ServerComConfig {
        &self.srv_com
    }

    pub fn server_com_config_mut(&mut self) -> &mut ServerComConfig {
        &mut self.srv_com
    }
}