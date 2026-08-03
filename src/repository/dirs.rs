pub fn default_download_dir() -> std::path::PathBuf {
    #[cfg(target_os = "linux")]
    return std::env::home_dir().unwrap().join("Downloads/mhserver-client");

    #[cfg(target_os = "windows")]
    return std::env::home_dir().unwrap().join("Downloads\\mhserver-client");
}