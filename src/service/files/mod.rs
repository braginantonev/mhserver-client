pub mod connections;
mod path;

use {
    super::ServiceError,
    crate::{config::files::FileServiceConfig, repository::ratelimit}, api::{
        apis::{Error, configuration::Configuration, default_api::*}, models::{ConnectionMode, ConnectionRequest, FilesListInner, SaveChunk},
    }, std::{fs::File, path::Path, sync::Arc}, system_interface::fs::FileIoExt, uuid::Uuid,
};

pub struct Size(i64);

impl Size {
    /// Return eye candy size. Example: `12 mb`, `1 gb` etc
    pub fn to_string_candy(&self) -> String {
        const EXTENSIONS: [&str; 5] = ["b", "kb", "mb", "gb", "tb"];
        let mut size = self.0 as f64;
        let mut i = 0;
        while size > 1024.0 && i < EXTENSIONS.len()-1 {
            size = size / 1024.0;
            i += 1;
        }
        format!("{} {}", size as f32, EXTENSIONS[i])
    }
}

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
    pub fn new() -> Self {
        Self { 
            cfg: FileServiceConfig::default(),
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

    /// Go to next folder, and return files list
    pub async fn next(&mut self, dir_name: &str) -> Result<Vec<FilesListInner>, ServiceError> {
        self.active_dir.push(dir_name);
        self.get_files(None).await
    }

    /// Go to previous folder, and return files list
    pub async fn prev(&mut self) -> Result<Vec<FilesListInner>, ServiceError> {
        self.active_dir.pop();
        self.get_files(None).await
    }

    pub async fn cancel_load(&mut self, id: Uuid) {
        self.connections.cancel(id).await;
    }

    //* API requests

    pub async fn available_space(&self) -> Result<Size, ServiceError> {
        self.queue.wait().await;
        match files_get_available_space(&self.cfg.api_conf).await {
            Ok(v) => Ok(Size(v.content.unwrap_or_default())),
            Err(err) => Err(ServiceError::from(err))
        }
    }

    pub async fn make_dir(&mut self, new_dir: &str) -> Result<(), ServiceError> {
        self.queue.wait().await;
        match files_make_directory(&self.cfg.api_conf, self.active_dir.with(new_dir).to_string().as_str()).await {
            Ok(_) => {
                // Append new dir to files list instead a send request to server, to reduce the load on it.
                self.cached_files.0.push(FilesListInner { name: new_dir.to_owned(), is_dir: Some(true), size: None, mod_time: 0 });
                Ok(())
            },
            Err(err) => Err(ServiceError::from(err))
        }
    }

    pub async fn remove_dir(&mut self, target_dir: &str) -> Result<(), ServiceError> {
        self.queue.wait().await;
        match files_remove_directory(&self.cfg.api_conf, self.active_dir.with(target_dir).to_string().as_str()).await {
            Ok(_) => {
                self.cached_files.remove(target_dir, true);
                Ok(())
            },
            Err(err) => Err(ServiceError::from(err))
        }
    }

    /// Get files list from server and save to local cache
    pub async fn get_files(&mut self, from: Option<String>) -> Result<Vec<FilesListInner>, ServiceError> {
        self.queue.wait().await;
        match get_files_list(&self.cfg.api_conf, &from.unwrap_or(self.current_dir())).await {
            Ok(res ) => {
                self.cached_files = FilesList(res.content.unwrap());
                Ok(self.cached_files())
            },
            Err(err) => match err {
                // If dir don't have files, serde understand this like null response on required field 
                Error::Serde(_) => {
                    self.cached_files = FilesList::default();
                    Ok(self.cached_files())
                }, 
                _ => Err(ServiceError::from(err))
            }
        }
    }

    pub async fn get_load_files(&self) -> Vec<connections::ConnectionInfo> {
        self.connections.progress_list().await
    }

    pub async fn get_loaded_files(&self) -> Vec<connections::CompletedLoad> {
        self.connections.completed().await
    }

    /// Save file to the server. That function return uuid like a String that can be used to get saving progress.
    pub async fn upload_file(&mut self, os_file_path: &Path) -> Result<Uuid, ServiceError> {
        let file = match File::open(os_file_path) {
            Ok(f) => Arc::new(f),
            Err(err) => return Err(ServiceError::new("failed upload file", Some(err.to_string()), None)),
        };

        let file_meta = match file.metadata() {
            Ok(m) => m,
            Err(err) => return Err(ServiceError::new("failed upload file", Some(err.to_string()), None)),
        };

        let filename = os_file_path.file_name().unwrap().display().to_string();

        let conn_req = ConnectionRequest {
            directory: self.active_dir.to_string(),
            filename: filename.clone(),
            size: Some(file_meta.len() as i64),
        };
        
        let save_info = match files_create_connection(&self.cfg.api_conf, ConnectionMode::Rdwr, conn_req).await {
            Ok(conn) => conn,
            Err(err) => return Err(ServiceError::from(err).with_label("failed upload file")),
        };

        let conn_info = save_info.content.unwrap();
        let conn_record = connections::ConnectionInner::new(filename, conn_info.chunks_count).upload_conn();
        let mut cancel_channel = conn_record.cancel_receiver();

        self.connections.add(conn_info.uuid, conn_record).await;
        let connections = self.connections.clone();

        let http_cfg = Arc::new(self.cfg.api_conf.clone());
        let rl_queue = self.queue.clone();

        // save file
        tokio::spawn(async move {
            for ch_idx in 0..conn_info.chunks_count {
                if cancel_channel.try_recv().is_ok() {
                    return
                }

                let offset = conn_info.chunk_size * ch_idx as i64;
                let file = file.clone();

                //todo: add semaphore to restrict a ram usage
                // read file part (chunk) to upload
                let chunk = tokio::task::spawn_blocking(move || {
                    let mut save_chunk = vec![0u8; conn_info.chunk_size as usize];
                    let read = file.read_at(save_chunk.as_mut_slice(), offset as u64).expect("failed read chunk from file");
                    save_chunk[..read].to_vec()
                });

                let mut connections = connections.clone();
                let http_cfg = http_cfg.clone();
                let mut cancel = cancel_channel.resubscribe();

                rl_queue.wait().await;
                tokio::spawn(async move {
                    if cancel.try_recv().is_ok() { return }
                    let chunk = chunk.await.expect("blocking file read failed"); // temp, hope

                    if cancel.try_recv().is_ok() { return }

                    match files_save_chunk(http_cfg.as_ref(), conn_info.uuid.to_string().as_str(), SaveChunk::new(chunk, offset)).await {
                        Ok(_) => {
                            connections.increase_progress(conn_info.uuid).await;
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

    pub async fn download_file(&mut self, from: Option<String>, filename: String) -> Result<Uuid, ServiceError> {
        let with_dirs = from.is_some();
        let from = from.unwrap_or(self.current_dir());

        let mut save_to = self.cfg.download_dir();
        if with_dirs {
            save_to = save_to.join(&from[1..]);
            if let Err(err) = std::fs::create_dir_all(save_to.as_path()) {
                return Err(ServiceError::new("failed download file", Some(err.to_string()), None).with_desc(&filename));
            }
        }

        let save_to = save_to.join(filename.clone() + ".part");
        let file = match File::create(save_to.as_path()) {
            Ok(f) => Arc::new(f),
            Err(err) => return Err(ServiceError::new("failed download file", Some(err.to_string()), None).with_desc(&filename)),
        };

        let conn_req = ConnectionRequest {
            directory: from,
            filename: filename.clone(),
            size: None,
        };
        
        let download_info = match files_create_connection(&self.cfg.api_conf, ConnectionMode::Rdonly, conn_req).await {
            Ok(conn) => conn,
            Err(err) => return Err(ServiceError::from(err)),
        };

        let download_info = download_info.content.unwrap();

        if let Err(err) = file.set_len(download_info.chunk_size as u64 * download_info.chunks_count as u64) {
            let _ = std::fs::remove_file(save_to.as_path());
            return Err(ServiceError::new("failed download file", Some(err.to_string()), None).with_desc(&filename));
        }

        let conn_record = connections::ConnectionInner::new(filename.clone(), download_info.chunks_count);
        let mut save_cancel = conn_record.cancel_receiver();
        let mut download_cancel = conn_record.cancel_receiver();

        self.connections.add(download_info.uuid, conn_record).await;

        let http_cfg = Arc::new(self.cfg.api_conf.clone());
        let rl_queue = self.queue.clone();

        let connections = self.connections.clone();
        let (tx, mut rx) = tokio::sync::mpsc::channel::<(Option<Vec<u8>>, i64)>(5); // tmp

        // download stage
        tokio::spawn(async move {
            for ch_idx in 0..download_info.chunks_count {
                rl_queue.wait().await;
                if download_cancel.try_recv().is_ok() { return }

                let http_cfg = http_cfg.clone();
                let tx = tx.clone();
                let mut connections = connections.clone();
                
                if download_cancel.try_recv().is_ok() { return }
                tokio::spawn(async move {
                    let offset = download_info.chunk_size * ch_idx as i64;
                    match files_get_chunk(&http_cfg, download_info.uuid.to_string().as_str(), ch_idx).await {
                        Ok(v) => {
                            connections.increase_progress(download_info.uuid).await;
                            let _ = tx.send((Some(v.content.unwrap().bytes().await.unwrap().to_vec()), offset)).await;
                        },
                        Err(err) => {
                            match err {
                                Error::ResponseError(c) => eprintln!("resp err: {}", c.content),
                                _ => eprintln!("err: {}", err),
                            }
                            let _ = tx.send((None, offset)).await;
                        }
                    }
                });
                if download_cancel.try_recv().is_ok() { return }
            }
        });

        // save stage
        tokio::spawn(async move {
            let mut canceled = false;
            let mut handles = Vec::with_capacity(download_info.chunks_count as usize);
            for _ in 0..download_info.chunks_count {
                let v = rx.recv().await.unwrap_or_default();
                let f = file.clone();

                if save_cancel.try_recv().is_ok() {
                    canceled = true;
                    break
                }
                
                handles.push(tokio::task::spawn_blocking(move || {
                    if let Some(chunk) = v.0 {
                        if let Err(err) = f.write_at(&chunk, v.1 as u64) {
                            eprintln!("failed write chunk to file ({err})")
                        }
                        
                    } else {
                        eprintln!("return a null chunk to write")
                    }
                }));
            };

            for h in handles {
                let _ = h.await;
            }

            if canceled {
                if std::fs::remove_file(save_to.as_path()).is_err() {
                    eprintln!("failed remove canceled download file");
                }
                return
            }

            if std::fs::rename(save_to.as_path(), save_to.with_file_name(filename)).is_err() {
                eprintln!("failed rename download file");
            };
        });

        Ok(download_info.uuid)
    }
}

impl super::Service for FileManager {
    fn update_config(&mut self, client: reqwest::Client, app_cfg: crate::config::app::ApplicationConfig) {
        let server_api_conf = app_cfg.server_api_config();
        let mut api_cfg = Configuration::new();
        api_cfg.client = client;
        api_cfg.base_path = server_api_conf.base_path().to_owned();
        api_cfg.bearer_access_token = Some(server_api_conf.jwt().to_owned());

        self.cfg = FileServiceConfig::new(api_cfg, app_cfg.download_dir())
    }
}