pub fn default_download_dir() -> std::path::PathBuf {
    std::env::home_dir().unwrap().join("Downloads/mhserver-client")
}