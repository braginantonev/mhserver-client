use {
    super::path::ServerPath,
    std::{path::{Path, PathBuf}, sync::Arc}, tokio::sync::mpsc,
};

pub fn default_download_dir() -> std::path::PathBuf {
    #[cfg(target_os = "linux")]
    return std::env::home_dir().unwrap().join("Downloads/mhserver-client");

    #[cfg(target_os = "windows")]
    return std::env::home_dir().unwrap().join("Downloads\\mhserver-client");
}

#[derive(Clone)]
pub struct Directory {
    name: String,
    os_path: PathBuf,
    files: Vec<String>,
    dirs: Vec<Arc<Directory>>
}

impl Directory {
    /// Recursive read directory, while new directories will not be founded.
    /// Return directory with another dirs and files, without name and os path.
    pub fn from_recursive(path: &Path) -> Self {
        let mut result = Self::empty();
        result.name = path.file_name().unwrap_or_default().to_str().unwrap().to_owned();
        result.os_path = path.to_path_buf();

        let dir = match path.read_dir() {
            Ok(dir) => dir,
            Err(_) => { return result; }
        };

        for entry in dir.filter_map(|x| x.ok()) {
            if let Ok(meta) = entry.metadata() {
                if meta.is_dir() {
                    result.dirs.push(Arc::new(Self::from_recursive(&entry.path())));
                } else {
                    result.files.push(entry.file_name().to_str().unwrap_or_default().to_owned());
                }
            }
        }

        result
    }

    pub fn empty() -> Self {
        Self { 
            name: String::new(),
            os_path: PathBuf::new(), 
            files: Vec::new(), 
            dirs: Vec::new() 
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn dirs(&self) -> Vec<Arc<Directory>> {
        self.dirs.clone()
    }

    pub fn read(self) -> mpsc::Receiver<(ServerPath, Vec<PathBuf>)> {
        let (tx, rx) = mpsc::channel(50);
        tokio::task::spawn_blocking(move || {
            let _ = self.read_recursive(ServerPath::new(), tx);
        });
        rx
    }

    fn read_recursive(&self, from: ServerPath, tx: mpsc::Sender<(ServerPath, Vec<PathBuf>)>) {
        let from = from.with(self.name());
        for dir in self.dirs() {
            let _ = dir.read_recursive(from.clone(), tx.clone());
        }
        let _ = tx.blocking_send((from, self.files.iter().map(|x| self.os_path.join(x)).collect()));
    }
}