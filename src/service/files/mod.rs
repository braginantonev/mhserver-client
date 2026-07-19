mod connections;
mod path;

use {
    crate::{NotificationType, actions::UiActions, config::files::FileServiceConfig, repository::ratelimit}, 
    api::{
        apis::{Error, default_api::*},
        models::{ConnectionMode, ConnectionRequest, FilesListInner, SaveChunk},
    }, 
    std::{fs::File, path::Path, sync::Arc}, 
    system_interface::fs::FileIoExt, 
    uuid::Uuid,
};

#[derive(Clone)]
struct FilesList(Vec<FilesListInner>);

impl FilesList {
    pub fn new() -> Self {
        FilesList(Vec::new())
    }

    pub fn back_dir(&self) -> FilesListInner {
        FilesListInner { name: "..".to_owned(), is_dir: Some(true), size: None, mod_time: 0 }
    }

    /// Return yourself but with back dir in start of vec
    pub fn with_back(&self) -> Self {
        let mut res = Vec::with_capacity(self.0.len() + 1);
        res.push(self.back_dir());
        res.extend_from_slice(&self.0);
        Self(res)
    }

    pub fn remove(&mut self, target: &str, is_dir: bool) {
        self.0.remove(self.0.iter().position(|f| f.name == target && f.is_dir.unwrap_or(false) == is_dir ).expect("dir to remove not found"));
    }
}

impl Default for FilesList {
    fn default() -> Self {
        FilesList::new()
    }
}


pub struct FileManager {
    cfg: FileServiceConfig,
    active_dir: path::ServerPath,
    cached_files: FilesList, // for files in active dir
    connections: connections::Connections,

    queue: Arc<ratelimit::RequestQueue>
}

impl FileManager {
    pub fn new(cfg: FileServiceConfig) -> Self {
        Self { 
            cfg,
            active_dir: path::ServerPath::new(),
            cached_files: FilesList::new(),
            connections: connections::Connections::new(),
            queue: Arc::new(ratelimit::RequestQueue::new(tokio::time::Duration::from_millis(50))), // tmp
        }
    }

    pub fn current_dir(&self) -> String {
        self.active_dir.to_string()
    }

    /// Get a cached files list 
    pub fn cached_files(&self) -> Vec<FilesListInner> {
        if self.current_dir() != "/" { self.cached_files.with_back().0 } else { self.cached_files.0.clone() }
    }

    /// Change file manager active directory. Client the guarantees that new_dir is exist on the server.
    async fn change_dir(&mut self, new_dir: path::ServerPath) -> Result<Vec<FilesListInner>, UiActions> {
        self.active_dir = new_dir;
        self.get_files().await
    }

    /// Go to next folder, and return files list
    pub async fn next(&mut self, dir_name: &str) -> Result<Vec<FilesListInner>, UiActions> {
        self.change_dir(self.active_dir.with(dir_name)).await
    }

    /// Go to previous folder, and return files list
    pub async fn prev(&mut self) -> Result<Vec<FilesListInner>, UiActions> {
        let mut target = self.active_dir.clone();
        target.pop();
        self.change_dir(target).await
    }

    //* API requests

    pub async fn available_space(&self) -> Result<i64, UiActions> {
        self.queue.wait().await;
        match files_get_available_space(&self.cfg.api_conf).await {
            Ok(v) => Ok(v.content.unwrap()),
            Err(err) => match err {
                Error::ResponseError(c) => Err(UiActions::ShowNotification(c.content, NotificationType::Error)),
                _ => Err(UiActions::ShowNotification(err.to_string(), NotificationType::Error)),
            }
        }
    }

    pub async fn make_dir(&mut self, new_dir: &str) -> Result<(), UiActions> {
        self.queue.wait().await;
        match files_make_directory(&self.cfg.api_conf, self.active_dir.with(new_dir).to_string().as_str()).await {
            Ok(_) => {
                // Append new dir to files list instead a send request to server, to reduce the load on it.
                self.cached_files.0.push(FilesListInner { name: new_dir.to_owned(), is_dir: Some(true), size: None, mod_time: 0 });
                Ok(())
            },
            Err(err) => {
                match err {
                    Error::ResponseError(c) => Err(UiActions::ShowNotification(c.content, NotificationType::Error)),
                    _ => Err(UiActions::ShowNotification(err.to_string(), NotificationType::Error)),
                }
            }
        }
    }

    pub async fn remove_dir(&mut self, target_dir: &str) -> Result<(), UiActions> {
        self.queue.wait().await;
        match files_remove_directory(&self.cfg.api_conf, self.active_dir.with(target_dir).to_string().as_str()).await {
            Ok(_) => {
                self.cached_files.remove(target_dir, true);
                Ok(())
            },
            Err(err) => match err {
                Error::ResponseError(c) => Err(UiActions::ShowNotification(c.content, NotificationType::Error)),
                _ => Err(UiActions::ShowNotification(err.to_string(), NotificationType::Error)),
            }
        }
    }

    /// Get files list from server and save to local cache
    pub async fn get_files(&mut self) -> Result<Vec<FilesListInner>, UiActions> {
        self.queue.wait().await;
        match get_files_list(&self.cfg.api_conf, self.current_dir().as_str()).await {
            Ok(res ) => {
                self.cached_files = FilesList(res.content.unwrap());
                Ok(self.cached_files())
            },
            Err(err) => {
                match err {
                    Error::ResponseError(c) => Err(UiActions::ShowNotification(c.content, NotificationType::Error)),
                    Error::Serde(_) => { // Null response
                        self.cached_files = FilesList::default();
                        Ok(self.cached_files())
                    }, 
                    _ => Err(UiActions::ShowNotification(err.to_string(), NotificationType::Error))
                }
            }
        }
    }

    pub fn get_load_files(&self) -> Vec<connections::FileProgress> {
        self.connections.progress_list()
    }

    /// Save file to the server. That function return uuid like a String that can be used to get saving progress.
    pub async fn upload_file(&mut self, os_file_path: &Path) -> Result<Uuid, UiActions> {
        let file = match File::open(os_file_path) {
            Ok(f) => Arc::new(f),
            Err(err) => return Err(UiActions::ShowNotification(err.to_string(), NotificationType::Error)),
        };

        let file_meta = match file.metadata() {
            Ok(m) => m,
            Err(err) => return Err(UiActions::ShowNotification(err.to_string(), NotificationType::Error)),
        };

        let filename = os_file_path.file_name().unwrap().display().to_string();

        let conn_req = ConnectionRequest {
            directory: self.active_dir.to_string(),
            filename: filename.clone(),
            size: Some(file_meta.len() as i64),
        };
        
        let save_info = match files_create_connection(&self.cfg.api_conf, ConnectionMode::Rdwr, conn_req).await {
            Ok(conn) => conn,
            Err(err) => return Err(match err {
                Error::ResponseError(c) => UiActions::ShowNotification(c.content, crate::NotificationType::Error),
                _ => UiActions::ShowNotification(err.to_string(), crate::NotificationType::Error),
            }),
        };

        let conn_info = save_info.content.unwrap();

        self.connections.add(conn_info.uuid, connections::ConnectionInner::new(filename, conn_info.chunk_size, conn_info.chunks_count));
        let connections = self.connections.clone();

        let http_cfg = Arc::new(self.cfg.api_conf.clone());
        let rl_queue = self.queue.clone();

        // save file
        tokio::spawn(async move {
            for ch_idx in 0..conn_info.chunks_count {
                let offset = conn_info.chunk_size * ch_idx as i64;
                let file = file.clone();

                // read file part (chunk) to upload
                let chunk = tokio::task::spawn_blocking(move || {
                    let mut save_chunk = vec![0u8; conn_info.chunk_size as usize];
                    let read = file.read_at(save_chunk.as_mut_slice(), offset as u64).expect("failed read chunk from file");
                    save_chunk[..read].to_vec()
                }).await.expect("blocking file read failed");

                let mut connections = connections.clone();
                let http_cfg = http_cfg.clone();

                rl_queue.wait().await;
                tokio::spawn(async move {
                    match files_save_chunk(http_cfg.as_ref(), conn_info.uuid.to_string().as_str(), SaveChunk::new(chunk, offset)).await {
                        Ok(_) => {
                            connections.increase_progress(conn_info.uuid);
                        },
                        Err(err) => match err {
                            Error::ResponseError(c) => println!("resp err: {}", c.content),
                            _ => println!("err: {}", err),
                        }
                    }
                });
            }
        });
        
        Ok(conn_info.uuid)
    }
}

impl super::Service for FileManager {
    fn update_config_from_app(&mut self, app_cfg: crate::config::app::ApplicationConfig) {
        let server_api_conf = app_cfg.server_api_config();

        self.cfg.api_conf.base_path = server_api_conf.base_path().to_owned();
        self.cfg.api_conf.bearer_access_token = Some(server_api_conf.jwt().to_owned());
    }
}