use std::path::PathBuf;
use crate::service::files::dirs::default_download_dir;

const DEFAULT_CHUNK_SIZE: usize = 1024 * 1024;

#[derive(Debug)]
pub struct FileServiceConfig {
    download_dir: PathBuf,

    // bytes in second
    server_throughput: usize,
    chunk_size: usize,
}

impl FileServiceConfig {
    pub fn new(download_dir: Option<PathBuf>, server_throughput: usize) -> Self {
        Self {
            download_dir: download_dir.unwrap_or(default_download_dir()),
            server_throughput,
            chunk_size: DEFAULT_CHUNK_SIZE,
        }
    }

    pub fn download_dir(&self) -> PathBuf {
        self.download_dir.clone()
    }

    pub fn server_throughput(&self) -> usize {
        self.server_throughput
    }

    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }
}

impl Default for FileServiceConfig {
    fn default() -> Self {
        Self {
            download_dir: default_download_dir(),
            server_throughput: 0,
            chunk_size: DEFAULT_CHUNK_SIZE,
        }
    }
}
