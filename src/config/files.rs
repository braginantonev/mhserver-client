use std::path::PathBuf;
use api::apis::configuration::Configuration;

use crate::repository::dirs::default_download_dir;

#[derive(Debug)]
pub struct FileServiceConfig {
    pub api_conf: Configuration,
    download_dir: PathBuf,
}

impl FileServiceConfig {
    pub fn new(api_conf: Configuration, download_dir: Option<PathBuf>) -> Self {
        Self { api_conf, download_dir: download_dir.unwrap_or(default_download_dir()) }
    }

    pub fn download_dir(&self) -> PathBuf {
        self.download_dir.clone()
    }
}

impl Default for FileServiceConfig {
    fn default() -> Self {
        Self { api_conf: Configuration::default(), download_dir: default_download_dir() }
    }
}