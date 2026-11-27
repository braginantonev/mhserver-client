pub mod loader;
pub mod dirs;
pub mod path;

use {
    super::ServiceError, 
    crate::{
        api::v3::{self as api, FilesList, files_service_client::FilesServiceClient},
        config::files::FileServiceConfig,
        repository::{
            ratelimit::StreamThrottle,
        },
    }, 
    std::{
        fs::File, 
        path::PathBuf, 
        sync::Arc
    }, 
    system_interface::fs::FileIoExt,
    tokio::sync::RwLock,
    tokio_stream::StreamExt,
};

type Client = FilesServiceClient<crate::repository::transport::Channel>;

impl FilesList {
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
        let mut res = Vec::with_capacity(self.value.len() + 1);
        res.push(self.back_dir());
        res.extend_from_slice(&self.value);
        Self { value: res }
    }

    // API 3.x not accept dir and file with same name, so we can don't check dir target or not
    pub fn remove(&mut self, target: path::ServerPath) -> Option<api::FileInfo> {
        let target = target.last()?;
        let target_idx = self.value
            .iter()
            .enumerate()
            .find(|x| x.1.name == *target)?
            .0;

        Some(self.value.remove(target_idx))
    }
}

#[derive(Default, Clone)]
struct ActiveDirectory {
    path: path::ServerPath,
    files: FilesList,
}

impl ActiveDirectory {
    pub fn path(&self) -> path::ServerPath {
        self.path.clone()
    }

    pub fn path_mut(&mut self) -> &mut path::ServerPath {
        &mut self.path
    }

    pub fn path_string(&self) -> String {
        self.path.to_string()
    }

    pub fn files(&self) -> FilesList {
        self.files.clone()
    }

    pub fn files_mut(&mut self) -> &mut FilesList {
        &mut self.files
    }

    pub fn set_files(&mut self, value: FilesList) {
        self.files = value;
    }
}

pub struct FileManager {
    cfg: FileServiceConfig,
    dirs_head: RwLock<ActiveDirectory>,
    loader: Arc<loader::Loader>,

    stream_throttle: StreamThrottle,
}

impl FileManager {
    pub fn new(cfg: FileServiceConfig) -> Self {
        let stream_throttle = StreamThrottle::new(cfg.server_throughput(), cfg.chunk_size());

        Self {
            cfg,
            dirs_head: RwLock::new(ActiveDirectory::default()),
            loader: Arc::default(),
            stream_throttle,
        }
    }

    pub async fn current_dir(&self) -> path::ServerPath {
        self.dirs_head.read().await.path()
    }

    /// Get a cached files list
    pub async fn cached_files(&self) -> FilesList {
        let lock = self.dirs_head.read().await;
        if lock.path_string() != "/" {
            lock.files().with_back()
        } else {
            lock.files().clone()
        }
    }

    /// Go to next folder, and return files list
    pub async fn next(
        &self,
        client: Client,
        dir_name: &str,
    ) -> Result<FilesList, ServiceError> {
        let target = {
            let mut lock = self.dirs_head.write().await;
            lock.path_mut().push(dir_name);
            lock.path()
        };
        
        // using Some(target) instead None to not lock dirs_head again
        self.get_files(client, Some(target)).await
    }

    /// Go to previous folder, and return files list
    pub async fn prev(&self, client: Client) -> Result<FilesList, ServiceError> {
        let target = {
            let mut lock = self.dirs_head.write().await;
            lock.path_mut().pop();
            lock.path()
        };

        // using Some(target) instead None to not lock dirs_head again
        self.get_files(client, Some(target)).await
    }

    pub async fn cd(&self, client: Client, target: path::ServerPath) -> Result<FilesList, ServiceError> {
        *self.dirs_head.write().await.path_mut() = target.clone();
        self.get_files(client, Some(target)).await
    }

    pub async fn cancel_load(&self, id: u64) {
        self.loader.cancel_load(id).await;
    }

    pub async fn get_load_files(&self) -> Vec<(loader::LoadID, loader::Load)> {
        self.loader.get_uncompleted().await
    }

    pub async fn get_loaded_files(&self) -> Vec<loader::Load> {
        self.loader.drain_completed().await
    }

    //* API requests

    pub async fn available_space(&self, mut client: Client) -> Result<usize, ServiceError> {
        match client.get_available_disk_space(()).await {
            Ok(v) => Ok(v.get_ref().value as usize),
            Err(err) => Err(ServiceError::from(err)),
        }
    }

    pub async fn make_dir(
        &self,
        mut client: Client,
        target: path::ServerPath
    ) -> Result<(), ServiceError> {
        let request = api::TargetDirectory {
            value: target.to_string(),
        };
            
        client.create_dir(request).await.map_err(|st| ServiceError::from(st))?;

        // Append new dir to files list instead a send request to server, to reduce the load on it.
        let dirname = match Into::<Vec<String>>::into(target).last() {
            Some(x) => x.clone(),
            None => return Ok(()),
        };

        let new_dir = api::FileInfo {
            name: dirname,
            is_dir: true,
            size: 0,
            mod_time: 0,
        };

        Ok(self.dirs_head.write().await.files_mut().value.push(new_dir))
    }

    pub async fn remove_dir(
        &self,
        mut client: Client,
        target: path::ServerPath,
    ) -> Result<(), ServiceError> {
        let request = api::TargetDirectory {
            value: target.to_string(),
        };

        client.remove_dir(request).await.map_err(|st| ServiceError::from(st))?; 
        
        // remove dir without get files from server
        if self.dirs_head.write().await.files_mut().remove(target).is_none() {
            return Err(ServiceError::new("Failed remove directory", Some("Target not found".to_owned()), None));
        };

        Ok(())
    }

    pub async fn remove_file(
        &self,
        mut client: Client,
        mut target: path::ServerPath,
    ) -> Result<(), ServiceError> {
        let filename = target.pop().unwrap_or_default();
        let request = api::TargetFile {
            directory: target.to_string(),
            name: filename.clone(),
        };

        client.remove_file(request).await.map_err(|st| ServiceError::from(st))?;

        // remove dir without get files from server
        if self.dirs_head.write().await.files_mut().remove(target.with(&filename)).is_none() {
            return Err(ServiceError::new("Failed remove file", Some("Target not found".to_owned()), None));
        };

        Ok(())
    }

    /// Get files list from server and save to local cache
    pub async fn get_files(
        &self,
        mut client: Client,
        from: Option<path::ServerPath>,
    ) -> Result<FilesList, ServiceError> {
        let from = from.unwrap_or(self.dirs_head.read().await.path());
        let request = api::TargetDirectory {
            value: from.to_string(),
        };

        let mut files = client.get_files(request).await.map_err(|st| ServiceError::from(st))?.into_inner(); 
        self.dirs_head.write().await.set_files(files.clone());

        if from.count() != 0 {
            files = files.with_back();
        }

        Ok(files)
    }

    /// Save file to the server. That function return JoinHandle with error, if he exist
    pub async fn upload_file(
        &self,
        mut client: Client,
        to: Option<path::ServerPath>,
        os_file_path: PathBuf,
    ) -> Result<(), ServiceError> {
        let upload_to: String = to.unwrap_or(self.dirs_head.read().await.path()).into();
        let filename = os_file_path.file_name().unwrap().display().to_string();

        if !path::dirpath_is_correct(&upload_to) {
            return Err(ServiceError::new("Bad directory syntax", None, None));
        }

        if !path::filepath_is_correct(&filename) {
            return Err(ServiceError::new("Bad filename syntax", None, None));
        }

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

        let load_id = self.loader.add(loader::Load::new(loader::LoadType::Upload, os_file_path.display().to_string(), Some(file_meta.len() as usize))).await;

        if file_meta.len() == 0 {
            return Err(ServiceError::new("Attempt to save empty file", None, None))
        }

        let conn_req = api::NewFile {
            target: Some(api::TargetFile {
                directory: upload_to,
                name: filename.clone()
            }),
            size: file_meta.len() as u64
        };

        let stream_loader = self.loader.clone();
        let chunk_size = self.cfg.chunk_size();
        let throttle = self.stream_throttle.clone();

        let (tx_stream, stream) = tokio::sync::mpsc::channel(10);

        tokio::spawn(async move {
            let mut offset: u64 = 0;

            if tx_stream.send(api::SaveFileRequest {
                info: Some(api::save_file_request::Info::Meta(conn_req)),
            }).await.is_err() {
                return;
            }

            while offset < file_meta.len() {
                let rl = tokio::time::sleep(throttle.duration());

                let mut save_chunk = vec![0u8; chunk_size];
                let read = file.read_at(save_chunk.as_mut_slice(), offset).expect("failed read chunk from file");

                rl.await;

                if tx_stream.send(api::SaveFileRequest {
                    info: Some(api::save_file_request::Info::Chunk(save_chunk[..read].to_vec()))
                }).await.is_err() {
                    return;
                };

                offset += chunk_size as u64;
                if !stream_loader.increase_loaded(load_id, read).await {
                    return;
                }
            }
        });

        let stream_loader = self.loader.clone();

        match client.save_file(tokio_stream::wrappers::ReceiverStream::new(stream)).await {
            Ok(_) => {
                stream_loader.complete_load(load_id).await;
                Ok(())
            },
            Err(status) => {
                stream_loader.drop_load(load_id).await;
                Err(ServiceError::from(status))
            }
        }
    }

    pub async fn download_file(
        &self,
        mut client: Client,
        from: Option<path::ServerPath>,
        target: String,
    ) -> Result<(), ServiceError> {
        let from: String = from.unwrap_or(self.dirs_head.read().await.path()).into();

        if !path::dirpath_is_correct(&from) {
            return Err(ServiceError::new("Bad directory syntax", None, None));
        }

        if !path::filepath_is_correct(&target) {
            return Err(ServiceError::new("Bad filename syntax", None, None));
        }

        let mut save_to = self.cfg.download_dir().to_path_buf();
        if from != "/" {
            save_to = save_to.join(&from);
        }

        let load_id = self.loader.add(loader::Load::new(loader::LoadType::Download, save_to.display().to_string(), None)).await;

        std::fs::create_dir_all(save_to.as_path())
            .map_err(|err| ServiceError::new("Failed download file", Some(err.to_string()), None)
                .with_desc(&from)
                .with_desc(&target))?; 

        let save_file = save_to.join(target.clone() + ".part");
        let file = File::create(save_file.as_path())
            .map_err(|err| ServiceError::new("Failed download file", Some(err.to_string()), None)
                .with_desc(&from)
                .with_desc(&target))?; 

        let conn_req = api::TargetFile {
            directory: from.clone(),
            name: target.clone(),
        };

        let mut stream = match client.read_file(conn_req).await {
            Ok(res) => res.into_inner(),
            Err(status) => return Err(ServiceError::from(status)),
        };

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
                download_err = Some(ServiceError::new("failed save file", Some(err.to_string()), None).with_desc(&save_file.display().to_string()));
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
                let _ = std::fs::rename(save_file.clone(), save_file.display().to_string().trim_end_matches(".part"));
                self.loader.complete_load(load_id).await;
                Ok(())
            },
        }
        
    }
}
