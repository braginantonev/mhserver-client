use {
    super::api::ServerApiConfiguration,
    serde::{Deserialize, Serialize},
    std::{env, fs, io, path::PathBuf},
};

const CONFIG_PATH: &str = ".config/mhserver-client";
const CONFIG_FILENAME: &str = "general.conf";
const PEM_CERT_FILENAME: &str = "rootCA.pem";

fn config_dir() -> PathBuf {
    env::home_dir()
        .expect("failed get home dir")
        .join(CONFIG_PATH)
}

fn config_file() -> PathBuf {
    config_dir().join(CONFIG_FILENAME)
}

pub fn read_pem_cert() -> io::Result<Vec<u8>> {
    fs::read(config_dir().join(PEM_CERT_FILENAME))
}

pub fn save_pem_cert(file: &str, over: bool) -> io::Result<()> {
    // we not use rename, because user can download pem file on other disk
    let body = fs::read(file)?;
    let target = config_dir().join(PEM_CERT_FILENAME);

    if !over && fs::exists(target.as_path())? {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "tls cert already exists"));
    }

    std::fs::write(target, body)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ApplicationConfig {
    download_directory: Option<String>,
    server_api: ServerApiConfiguration,
    server_throughput: usize,
}

impl ApplicationConfig {
    pub fn from_file() -> Result<Self, std::io::Error> {
        match fs::read_to_string(config_file()) {
            Ok(read) => {
                Ok(toml::from_str(read.as_str()).unwrap_or_default())
            }
            Err(err) => Err(err),
        }
    }

    pub fn save_to_file(&self) {
        if !fs::exists(config_file()).unwrap() {
            let _ = fs::create_dir_all(config_dir());
        }

        let _ = fs::write(
            config_file(),
            toml::to_string_pretty(&self).expect("failed serialize config to string"),
        );
    }

    pub fn server_api_config(&self) -> &ServerApiConfiguration {
        &self.server_api
    }

    pub fn server_api_config_mut(&mut self) -> &mut ServerApiConfiguration {
        &mut self.server_api
    }

    pub fn download_dir(&self) -> Option<PathBuf> {
        if let Some(dir) = self.download_directory.clone() {
            return Some(PathBuf::from(dir));
        }
        None
    }

    pub fn server_throughput(&self) -> usize {
        self.server_throughput
    }
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        Self { 
            download_directory: None, 
            server_api: ServerApiConfiguration::default(), 
            server_throughput: 200 * 1024 * 1024 
        }
    }
}
