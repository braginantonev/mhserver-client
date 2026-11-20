use std::path::{Path, PathBuf};

const DEFAULT_CHUNK_SIZE: usize = 1024 * 1024;

#[derive(Debug)]
pub struct FileServiceConfig {
    download_dir: PathBuf,

    // bytes in second
    server_throughput: usize,
    chunk_size: usize,
}

impl FileServiceConfig {
    pub fn new(download_dir: &Path, server_throughput: usize) -> Self {
        Self {
            download_dir: download_dir.to_path_buf(),
            server_throughput,
            chunk_size: DEFAULT_CHUNK_SIZE,
        }
    }

    pub fn download_dir(&self) -> &Path {
        &self.download_dir
    }

    pub fn server_throughput(&self) -> usize {
        self.server_throughput
    }

    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }
}