pub mod connections;
mod path;
pub mod dirs;

use {
    super::ServiceError, crate::{api::{self, data_service_client::DataServiceClient}, config::files::FileServiceConfig}, std::{fs::File, path::PathBuf, sync::Arc}, system_interface::fs::FileIoExt, tokio::task::JoinHandle, tokio_stream::StreamExt, tonic::transport::Channel, uuid::Uuid,
};

pub struct Size(u64);

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
struct FilesList(Vec<api::FileInfo>);

impl FilesList {
    pub fn new() -> Self {
        FilesList(Vec::new())
    }

    pub fn back_dir(&self) -> api::FileInfo {
        api::FileInfo { name: "..".to_owned(), is_dir: true, size: 0, mod_time: 0 }
    }

    /// Return yourself but with back dir in start of vec
    pub fn with_back(&self) -> Self {
        let mut res = Vec::with_capacity(self.0.len() + 1);
        res.push(self.back_dir());
        res.extend_from_slice(&self.0);
        Self(res)
    }

    pub fn remove(&mut self, target: &str, is_dir: bool) {
        self.0.remove(self.0.iter().position(|f| f.name == target && f.is_dir == is_dir).expect("dir to remove not found"));
    }
}

impl Default for FilesList {
    fn default() -> Self {
        FilesList::new()
    }
}

pub struct FileManager {
    cfg: FileServiceConfig,
    client: DataServiceClient<Channel>,
    active_dir: path::ServerPath,
    cached_files: FilesList, // for files in active dir
    connections: connections::Connections,
}

impl FileManager {
    pub fn new(cfg: FileServiceConfig, client: DataServiceClient<Channel>) -> Self {
        Self { 
            cfg,
            client,
            active_dir: path::ServerPath::new(),
            cached_files: FilesList::new(),
            connections: connections::Connections::new(),
        }
    }

    pub fn current_dir(&self) -> path::ServerPath {
        self.active_dir.clone()
    }

    /// Get a cached files list 
    pub fn cached_files(&self) -> Vec<api::FileInfo> {
        if self.active_dir.to_string() != "/" { self.cached_files.with_back().0 } else { self.cached_files.0.clone() }
    }

    /// Go to next folder, and return files list
    pub async fn next(&mut self, dir_name: &str) -> Result<Vec<api::FileInfo>, ServiceError> {
        self.active_dir.push(dir_name);
        self.get_files(None).await
    }

    /// Go to previous folder, and return files list
    pub async fn prev(&mut self) -> Result<Vec<api::FileInfo>, ServiceError> {
        self.active_dir.pop();
        self.get_files(None).await
    }

    pub async fn cancel_load(&mut self, id: Uuid) {
        self.connections.cancel(id).await;
    }

    //* API requests

    pub async fn available_space(&mut self) -> Result<Size, ServiceError> {
        match self.client.get_available_disk_space(()).await {
            Ok(v) => Ok(Size(v.get_ref().value)),
            Err(err) => Err(ServiceError::from(err))
        }
    }

    pub async fn make_dir(&mut self, from: Option<path::ServerPath>, new_dir: &str) -> Result<(), ServiceError> {
        let from = from.unwrap_or(self.current_dir());
        match self.client.create_dir(api::Directory { value: from.with(new_dir).to_string() }).await {
            Ok(_) => {
                // Append new dir to files list instead a send request to server, to reduce the load on it.
                self.cached_files.0.push(api::FileInfo { name: new_dir.to_owned(), is_dir: true, size: 0, mod_time: 0 });
                Ok(())
            },
            Err(err) => Err(ServiceError::from(err))
        }
    }

    pub async fn remove_dir(&mut self, target_dir: &str) -> Result<(), ServiceError> {
        match self.client.remove_dir(api::Directory { value: self.current_dir().with(target_dir).to_string() }).await {
            Ok(_) => {
                self.cached_files.remove(target_dir, true);
                Ok(())
            },
            Err(err) => Err(ServiceError::from(err))
        }
    }

    /// Get files list from server and save to local cache
    pub async fn get_files(&mut self, from: Option<String>) -> Result<Vec<api::FileInfo>, ServiceError> {
        match self.client.get_files(api::Directory { value: from.unwrap_or(self.current_dir().into()) }).await {
            Ok(res ) => {
                self.cached_files = FilesList(res.get_ref().value.clone());
                Ok(self.cached_files())
            },
            Err(err) => Err(ServiceError::from(err))
        }
    }

    pub async fn get_load_files(&self) -> Vec<connections::ConnectionInfo> {
        self.connections.progress_list().await
    }

    pub async fn get_loaded_files(&self) -> Vec<connections::CompletedLoad> {
        self.connections.completed().await
    }

    /// Save file to the server. That function return JoinHandle with error, if he exist
    pub fn upload_file(&mut self, to: Option<path::ServerPath>, os_file_path: PathBuf) -> JoinHandle<Result<(), ServiceError>> {
        let upload_to = to.unwrap_or(self.current_dir()).into();
        let mut connections = self.connections.clone();
        let mut client = self.client.clone();

        // save file
        tokio::spawn(async move {
            let filename = os_file_path.file_name().unwrap().display().to_string();
            let from = os_file_path.to_str().unwrap_or_default();

            let file = match File::open(os_file_path.clone()) {
                Ok(f) => Arc::new(f),
                Err(err) => return Err(ServiceError::new("failed upload file", Some(err.to_string()), None)),
            };

            let file_meta = match file.metadata() {
                Ok(m) => m,
                Err(err) => return Err(ServiceError::new("failed upload file", Some(err.to_string()), None)),
            };

            let conn_req = api::RequiredFile {
                dir: Some(api::Directory { value: upload_to }),
                name: filename.clone(),
                new_size: Some(file_meta.len() as u64),
            };

            let conn_record = connections::ConnectionInner::new(filename, todo!()).upload_conn();
            let cancel_token = conn_record.cancel_token();

            connections.add(todo!(), conn_record).await;
            let mut offset: u64 = 0;
            
            let stream = async_stream::stream! {
                const CHUNK_SIZE: usize = 1024 * 1024;
                yield api::SaveFileChunk {
                    info: Some(api::save_file_chunk::Info::Meta(conn_req)),
                };

                loop {
                    if cancel_token.is_cancelled() {
                        return;
                    }
        
                    let mut save_chunk = vec![0u8; CHUNK_SIZE];
                    let read = file.read_at(save_chunk.as_mut_slice(), offset).expect("failed read chunk from file");

                    yield api::SaveFileChunk {
                        info: Some(api::save_file_chunk::Info::Chunk(api::Chunk {
                            data: save_chunk[..read].to_vec(),
                            offset,
                        }))
                    };

                    offset += CHUNK_SIZE as u64;
                }
            };

            if let Err(status) = client.save_file(stream).await {
                return Err(ServiceError::from(status));
            }

            Ok(())
        })
    }

    pub fn download_file(&mut self, from: Option<String>, filename: String) -> JoinHandle<Result<(), ServiceError>> {
        let client = self.client.clone();
        let with_dirs = from.is_some();
        let from = from.unwrap_or(self.current_dir().into());

        let mut save_to = self.cfg.download_dir();
        if with_dirs {
            save_to = save_to.join(&from[1..]);
        }

        let mut connections = self.connections.clone();

        tokio::spawn(async move {
            if let Err(err) = std::fs::create_dir_all(save_to.as_path()) {
                return Err(ServiceError::new("failed download file", Some(err.to_string()), None).with_desc(&from).with_desc(&filename));
            }

            let save_to = save_to.join(filename.clone() + ".part");
            let file = match File::create(save_to.as_path()) {
                Ok(f) => f,
                Err(err) => return Err(ServiceError::new("failed download file", Some(err.to_string()), None).with_desc(&from).with_desc(&filename)),
            };

            let conn_req = api::RequiredFile {
                dir: Some(api::Directory{ value: from.clone() }),
                name: filename.clone(),
                new_size: None,
            };
            
            let conn_record = connections::ConnectionInner::new(filename.clone(), todo!());
            let cancel_token = conn_record.cancel_token();

            connections.add(todo!(), conn_record).await;

            let stream = match client.read_file(conn_req).await {
                Ok(res) => res.into_inner(),
                Err(status) => return Err(ServiceError::from(status))
            };

            while let Some(chunk) = stream.next().await {
                let chunk = match chunk {
                    Ok(c) => c,
                    Err(status) => return Err(ServiceError::from(status)),
                };
                file.write_at(&chunk.data, chunk.offset);
            };

            Ok(())
        })
    }
}
