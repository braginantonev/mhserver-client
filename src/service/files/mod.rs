pub mod loader;
pub mod dirs;
mod path;

use {
    super::ServiceError, 
    crate::{
        api::{self, data_service_client::DataServiceClient},
        config::files::FileServiceConfig,
    }, 
    std::{
        fs::File, 
        path::PathBuf, 
        sync::Arc
    }, 
    system_interface::fs::FileIoExt, 
    tokio_stream::StreamExt,
};

const CHUNK_SIZE: usize = 1024 * 1024;

type Client = DataServiceClient<crate::repository::transport::Channel>;

#[derive(Clone)]
pub struct Size(usize);

impl Size {
    /// Return eye candy size. Example: `12 mb`, `1 gb` etc
    pub fn to_string_candy(&self) -> String {
        const EXTENSIONS: [&str; 5] = ["b", "kb", "mb", "gb", "tb"];
        let mut size = self.0 as f32;
        let mut i = 0;
        while size > 1024.0 && i < EXTENSIONS.len() - 1 {
            size = size / 1024.0;
            i += 1;
        }
        format!("{} {}", size as f32, EXTENSIONS[i])
    }
}

impl Into<usize> for Size {
    fn into(self) -> usize {
        self.0
    }
}

#[derive(Clone)]
struct FilesList(Vec<api::FileInfo>);

impl FilesList {
    pub fn new() -> Self {
        FilesList(Vec::new())
    }

    pub fn back_dir(&self) -> api::FileInfo {
        api::FileInfo {
            name: "..".to_owned(),
            is_dir: true,
            size: 0,
            mod_time: 0,
        }
    }

    /// Return yourself but with back dir in start of vec
    pub fn with_back(&self) -> Self {
        let mut res = Vec::with_capacity(self.0.len() + 1);
        res.push(self.back_dir());
        res.extend_from_slice(&self.0);
        Self(res)
    }

    pub fn remove(&mut self, target: &str, is_dir: bool) {
        self.0.remove(
            self.0
                .iter()
                .position(|f| f.name == target && f.is_dir == is_dir)
                .expect("dir to remove not found"),
        );
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
    loader: Arc<loader::Loader>,
}

impl FileManager {
    pub fn new(cfg: FileServiceConfig) -> Self {
        Self {
            cfg,
            active_dir: path::ServerPath::new(),
            cached_files: FilesList::new(),
            loader: Arc::default(),
        }
    }

    pub fn current_dir(&self) -> path::ServerPath {
        self.active_dir.clone()
    }

    /// Get a cached files list
    pub fn cached_files(&self) -> Vec<api::FileInfo> {
        if self.active_dir.to_string() != "/" {
            self.cached_files.with_back().0
        } else {
            self.cached_files.0.clone()
        }
    }

    /// Go to next folder, and return files list
    pub async fn next(
        &mut self,
        client: Client,
        dir_name: &str,
    ) -> Result<Vec<api::FileInfo>, ServiceError> {
        self.active_dir.push(dir_name);
        self.get_files(client, None).await
    }

    /// Go to previous folder, and return files list
    pub async fn prev(&mut self, client: Client) -> Result<Vec<api::FileInfo>, ServiceError> {
        self.active_dir.pop();
        self.get_files(client, None).await
    }

    pub async fn cancel_load(&mut self, id: u64) {
        self.loader.cancel_load(id).await;
    }

    //* API requests

    pub async fn available_space(&self, mut client: Client) -> Result<Size, ServiceError> {
        match client.get_available_disk_space(()).await {
            Ok(v) => Ok(Size(v.get_ref().value as usize)),
            Err(err) => Err(ServiceError::from(err)),
        }
    }

    pub async fn make_dir(
        &mut self,
        mut client: Client,
        from: Option<path::ServerPath>,
        new_dir: &str,
    ) -> Result<(), ServiceError> {
        let from = from.unwrap_or(self.current_dir());
        match client
            .create_dir(api::Directory {
                value: from.with(new_dir).to_string(),
            })
            .await
        {
            Ok(_) => {
                // Append new dir to files list instead a send request to server, to reduce the load on it.
                self.cached_files.0.push(api::FileInfo {
                    name: new_dir.to_owned(),
                    is_dir: true,
                    size: 0,
                    mod_time: 0,
                });
                Ok(())
            }
            Err(err) => Err(ServiceError::from(err)),
        }
    }

    pub async fn remove_dir(
        &mut self,
        mut client: Client,
        target_dir: &str,
    ) -> Result<(), ServiceError> {
        let request = api::Directory {
            value: self.current_dir().with(target_dir).to_string(),
        };

        match client.remove_dir(request).await {
            Ok(_) => {
                self.cached_files.remove(target_dir, true); // remove dir without get files from server
                Ok(())
            }
            Err(err) => Err(ServiceError::from(err)),
        }
    }

    /// Get files list from server and save to local cache
    pub async fn get_files(
        &mut self,
        mut client: Client,
        from: Option<String>,
    ) -> Result<Vec<api::FileInfo>, ServiceError> {
        let request = api::Directory {
            value: from.unwrap_or(self.current_dir().to_string()),
        };

        match client.get_files(request).await {
            Ok(res) => {
                self.cached_files = FilesList(res.get_ref().value.clone());
                Ok(self.cached_files())
            }
            Err(err) => Err(ServiceError::from(err)),
        }
    }

    pub async fn get_load_files(&self) -> Vec<(loader::LoadID, loader::Load)> {
        self.loader.get_uncompleted().await
    }

    pub async fn get_loaded_files(&self) -> Vec<loader::Load> {
        self.loader.drain_completed().await
    }

    /// Save file to the server. That function return JoinHandle with error, if he exist
    pub async fn upload_file(
        &mut self,
        mut client: Client,
        to: Option<path::ServerPath>,
        os_file_path: PathBuf,
    ) -> Result<(), ServiceError> {
        let upload_to = to.unwrap_or(self.current_dir()).into();
        let filename = os_file_path.file_name().unwrap().display().to_string();

        let file = match File::open(os_file_path.clone()) {
            Ok(f) => Arc::new(f),
            Err(err) => {
                return Err(ServiceError::new(
                    "failed upload file",
                    Some(err.to_string()),
                    None,
                ))
            }
        };

        let file_meta = match file.metadata() {
            Ok(m) => m,
            Err(err) => {
                return Err(ServiceError::new(
                    "failed upload file",
                    Some(err.to_string()),
                    None,
                ))
            }
        };

        let conn_req = api::RequiredFile {
            dir: Some(api::Directory { value: upload_to }),
            name: filename.clone(),
            new_size: Some(file_meta.len() as u64),
        };

        let load_id = self.loader.add(loader::Load::new(loader::LoadType::Upload, os_file_path.display().to_string(), Some(Size(file_meta.len() as usize)))).await;

        let stream_loader = self.loader.clone();
        let mut offset: u64 = 0;
        let stream = async_stream::stream! {
            yield api::SaveFileChunk {
                info: Some(api::save_file_chunk::Info::Meta(conn_req)),
            };

            while offset < file_meta.len() {
                let mut save_chunk = vec![0u8; CHUNK_SIZE];
                let read = file.read_at(save_chunk.as_mut_slice(), offset).expect("failed read chunk from file");

                yield api::SaveFileChunk {
                    info: Some(api::save_file_chunk::Info::Chunk(api::Chunk {
                        data: save_chunk[..read].to_vec(),
                        offset,
                    }))
                };

                offset += CHUNK_SIZE as u64;
                if !stream_loader.increase_loaded(load_id, read).await {
                    break
                }
            }
        };

        match client.save_file(stream).await {
            Ok(_) => {
                self.loader.complete_load(load_id).await;
                Ok(())
            },
            Err(status) => {
                self.loader.drop_load(load_id).await;
                Err(ServiceError::from(status))
            }
        }
    }

    pub async fn download_file(
        &mut self,
        mut client: Client,
        from: Option<String>,
        filename: String,
    ) -> Result<(), ServiceError> {
        let with_dirs = from.is_some();
        let from = from.unwrap_or(self.current_dir().into());

        let mut save_to = self.cfg.download_dir();
        if with_dirs {
            save_to = save_to.join(&from[1..]);
        }

        if let Err(err) = std::fs::create_dir_all(save_to.as_path()) {
            return Err(
                ServiceError::new("failed download file", Some(err.to_string()), None)
                    .with_desc(&from)
                    .with_desc(&filename),
            );
        }

        let save_to = save_to.join(filename.clone() + ".part");
        let file = match File::create(save_to.as_path()) {
            Ok(f) => f,
            Err(err) => {
                return Err(
                    ServiceError::new("failed download file", Some(err.to_string()), None)
                        .with_desc(&from)
                        .with_desc(&filename),
                )
            }
        };

        let conn_req = api::RequiredFile {
            dir: Some(api::Directory {
                value: from.clone(),
            }),
            name: filename.clone(),
            new_size: None,
        };

        let mut stream = match client.read_file(conn_req).await {
            Ok(res) => res.into_inner(),
            Err(status) => return Err(ServiceError::from(status)),
        };

        let load_id = self.loader.add(loader::Load::new(loader::LoadType::Download, save_to.display().to_string(), None)).await; 

        let mut download_err: Option<ServiceError> = None;
        while let Some(chunk) = stream.next().await {
            let chunk = match chunk {
                Ok(c) => c,
                Err(status) => {
                    download_err = Some(ServiceError::from(status));
                    break
                }
            };

            if let Err(err) = file.write_at(&chunk.data, chunk.offset) {
                download_err = Some(ServiceError::new("failed save file", Some(err.to_string()), None).with_desc(&filename));
                break
            }

            if !self.loader.increase_loaded(load_id, chunk.data.len()).await {
                break
            }
        }

        match download_err {
            Some(err) => {
                self.loader.drop_load(load_id).await;
                Err(err)
            }
            None => {
                self.loader.complete_load(load_id).await;
                Ok(())
            },
        }
    }
}
