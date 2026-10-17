use std::path::PathBuf;

use crate::service::files::dirs::default_download_dir;

#[derive(Debug)]
pub struct FileServiceConfig {
    download_dir: PathBuf,
}

impl FileServiceConfig {
    pub fn new(download_dir: Option<PathBuf>) -> Self {
        Self { download_dir: download_dir.unwrap_or(default_download_dir()) }
    }

    pub fn download_dir(&self) -> PathBuf {
        self.download_dir.clone()
    }
}

impl Default for FileServiceConfig {
    fn default() -> Self {
        Self { download_dir: default_download_dir() }
    }
}