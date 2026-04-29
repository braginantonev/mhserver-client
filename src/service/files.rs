use {
    crate::{
        NotificationType, 
        actions::UiActions, 
        config::files::FileServiceConfig
    }, openapi::{
        apis::{ 
            Error, default_api::{
                files_create_connection, files_make_directory, files_remove_directory, files_save_chunk, get_files_list
            }
        },
        models::{ConnectionMode, ConnectionRequest, FilesListInner, SaveChunk},
    }, std::{collections::HashMap, fs::File, path::Path, sync::{Arc, Mutex}, thread, time}, system_interface::fs::FileIoExt, uuid::Uuid
};

#[derive(Clone)]
struct ServerPath {
    buff: Vec<String>,
}

impl ServerPath {
    pub fn new() -> Self {
        Self { buff: Vec::new() }
    }

    /// Push a single directory.
    #[allow(dead_code)] // Todo: Remove in prod
    pub fn push(&mut self, path: &str) {
        self.buff.push(path.to_owned());
    }

    /// Pop a singe directory.
    pub fn pop(&mut self) -> bool {
        self.buff.pop().is_some()
    }

    pub fn with(&self, element: &str) -> Self {
        let mut res = self.buff.clone();
        res.push(element.to_owned());
        Self { buff: res }
    }
}

impl ToString for ServerPath {
    fn to_string(&self) -> String {
        let mut s = String::from("/");
        for element in &self.buff {
            s.push_str(element);
            s.push('/');
        }
        s
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

// pub enum ConnectionState {
//     Active,
//     Completed,
//     Dropped
// }

struct ConnectionInner {
    chunk_size: i32,
    chunks_count: i32,
    loaded: i32, // count of saved or loaded chunks  
}

impl ConnectionInner {
    pub fn new(chunk_size: i32, chunks_count: i32) -> Self {
        Self { chunk_size, chunks_count, loaded: 0 }
    }
}

#[derive(Clone)]
struct Connections {
    inner: Arc<Mutex<HashMap<Uuid, ConnectionInner>>>
}

impl Connections {
    pub fn new() -> Self {
        Self { inner: Arc::new(Mutex::new(HashMap::new())) }
    }

    pub fn add(&mut self, key: Uuid, val: ConnectionInner) {
        self.inner.lock().unwrap().insert(key, val);
    }

    pub fn progress(&self, id: Uuid) -> f32 {
        let lock = self.inner.lock().unwrap();
        lock[&id].loaded as f32 / lock[&id].chunks_count as f32
    }

    pub fn increase_progress(&mut self, id: Uuid) -> bool {
        if let Some(conn) = self.inner.lock().unwrap().get_mut(&id) {
            conn.loaded += 1;
            return true;
        }
        false
    }
}

pub struct FileManager {
    cfg: FileServiceConfig,
    active_dir: ServerPath,
    cached_files: FilesList, // for files in active dir
    connections: Connections
}

impl FileManager {
    pub fn new(cfg: FileServiceConfig) -> Self {
        Self { 
            cfg,
            active_dir: ServerPath::new(),
            cached_files: FilesList::new(),
            connections: Connections::new(),
        }
    }

    /// Change file manager active directory. Client the guarantees that new_dir is exist on the server.
    async fn change_dir(&mut self, new_dir: ServerPath) -> Result<Vec<FilesListInner>, UiActions> {
        self.active_dir = new_dir;
        self.get_files().await
    }

    pub fn current_dir(&self) -> String {
        self.active_dir.to_string()
    }

    /// Get a cached files list
    pub fn cached_files(&self) -> Vec<FilesListInner> {
        if self.current_dir() != "/" { self.cached_files.with_back().0 } else { self.cached_files.0.clone() }
    }

    /// Get files list from server and save to local cache
    pub async fn get_files(&mut self) -> Result<Vec<FilesListInner>, UiActions> {
        match get_files_list(&self.cfg.api_conf, self.current_dir().as_str()).await {
            Ok(res ) => {
                self.cached_files = FilesList(res);
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

    pub async fn make_dir(&mut self, new_dir: &str) -> Result<(), UiActions> {
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

        let conn_req = ConnectionRequest {
            directory: self.active_dir.to_string(),
            filename: os_file_path.file_name().unwrap().display().to_string(),
            size: Some(file_meta.len() as i32),
        };

        println!("connection request: {:?}", conn_req);
        
        let save_info = match files_create_connection(&self.cfg.api_conf, conn_req, ConnectionMode::Rdwr).await {
            Ok(conn) => conn,
            Err(err) => return Err(UiActions::ShowNotification(err.to_string(), NotificationType::Error)),
        };

        println!("create connection: {:?}", save_info);

        self.connections.add(save_info.uuid, ConnectionInner::new(save_info.chunk_size, save_info.chunks_count));
        let connections = self.connections.clone();

        let http_cfg = Arc::new(self.cfg.api_conf.clone());

        tokio::spawn(async move {
            for ch_idx in 0..save_info.chunks_count {
                let offset = save_info.chunk_size as u64 * ch_idx as u64;
                let file = file.clone();

                let chunk = tokio::task::spawn_blocking(move || {
                    let mut save_chunk = vec![0u8; save_info.chunk_size as usize];
                    let read = file.read_at(save_chunk.as_mut_slice(), offset).expect("failed read chunk from file");
                    save_chunk[..read].to_vec()
                }).await.expect("blocking file read failed");

                let mut connections = connections.clone();
                let http_cfg = http_cfg.clone();

                println!("gg: {}", save_info.uuid.to_string());
                
                tokio::spawn(async move {
                    match files_save_chunk(http_cfg.as_ref(), SaveChunk::new(chunk, offset as i32), save_info.uuid.to_string().as_str()).await {
                        Ok(_) => {
                            connections.increase_progress(save_info.uuid);
                            println!("save {}", ch_idx);
                        },
                        Err(err) =>  match err {
                            Error::ResponseError(c) => println!("resp err: {}", c.content),
                            _ => println!("err: {}", err),
                        }
                    }
                });
                
                // Todo: Check in prod
                if ch_idx % 100 == 0 {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        });

        Ok(save_info.uuid)
    }
}

impl super::Service for FileManager {
    fn update_config_from_app(&mut self, app_cfg: crate::config::app::ApplicationConfig) {
        let server_api_conf = app_cfg.server_api_config();

        self.cfg.api_conf.base_path = server_api_conf.base_path().to_owned();
        self.cfg.api_conf.bearer_access_token = Some(server_api_conf.jwt().to_owned());
    }
}