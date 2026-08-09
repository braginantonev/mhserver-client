use std::path::{Path, PathBuf};

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
    dirs: Vec<Directory>
}

impl Directory {
    pub fn new(name: &str, path: &Path) -> Self {
        Self { 
            name: name.to_owned(), 
            os_path: PathBuf::from(path), 
            files: Vec::new(), 
            dirs: Vec::new() 
        }
    }

    pub fn read(&mut self) {
        if let Some(dir) = read_local_dir(&self.path()) {
            self.dirs = dir.dirs;
            self.files = dir.files;
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn path(&self) -> &Path {
        &self.os_path
    }

    pub fn files(&self) -> Vec<String> {
        self.files.clone()
    }

    pub fn dirs(&self) -> Vec<Directory> {
        self.dirs.clone()
    }
}

fn read_local_dir(path: &Path) -> Option<Directory> {
    let dir = match path.read_dir() {
        Ok(dir) => dir,
        Err(_) => { return None; }
    };

    let mut result = Directory::new(path.file_name().unwrap_or_default().to_str().unwrap_or("undefined"), path);

    for entry in dir.filter_map(|x| x.ok()) {
        if let Ok(meta) = entry.metadata() {
            if meta.is_dir() {
                if let Some(dir) = read_local_dir(&entry.path()) {
                    result.dirs.push(dir);
                }
            } else {
                result.files.push(entry.file_name().to_str().unwrap_or_default().to_owned());
            }
        }
    }

    Some(result)
}