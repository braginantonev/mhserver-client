use {
    crate::{
        NotificationType, 
        actions::UiActions, 
        config::files::FileServiceConfig
    }, 
    openapi::{
        apis::{ 
            default_api::{
                get_files_list,
                files_make_directory,
            },
            Error,
        },
        models::FilesListInner,
    }, 
    std::{
        path::{Path, PathBuf},
        str::FromStr
    }
};

// Temporary. API required
fn format_dir_for_req(target_dir: &str) -> String {
    format!("{}/", target_dir)
}

#[derive(Clone)]
pub struct FilesList(Vec<FilesListInner>);

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
}

impl Default for FilesList {
    fn default() -> Self {
        FilesList(Vec::new())
    }
}

pub struct FileManager {
    cfg: FileServiceConfig,
    active_dir: PathBuf,
    cached_files: FilesList,
}

impl FileManager {
    pub fn new(cfg: FileServiceConfig) -> Self {
        Self { 
            cfg,
            active_dir: PathBuf::from_str("/").unwrap(),
            cached_files: FilesList::new(),
        }
    }

    /// Change file manager active directory. Client the guarantees that target_dir is exist on the server.
    async fn change_dir(&mut self, target_dir: PathBuf) -> Result<Vec<FilesListInner>, UiActions> {
        self.active_dir = target_dir;
        self.get_files().await
    }

    pub fn get_current_dir(&self) -> &str {
        self.active_dir.to_str().unwrap_or("/")
    }

    /// Get a cached files list
    pub fn get_cached_files(&self) -> Vec<FilesListInner> {
        if self.get_current_dir() != "/" { self.cached_files.with_back().0 } else { self.cached_files.0.clone() }
    }

    /// Get files list from server and save to local cache
    pub async fn get_files(&mut self) -> Result<Vec<FilesListInner>, UiActions> {
        let mut req_dir = self.active_dir.to_str().unwrap().to_owned();

        if !req_dir.ends_with('/') {
            req_dir += "/";
        }

        match get_files_list(&self.cfg.api_conf, req_dir.as_str()).await {
            Ok(res ) => {
                self.cached_files = FilesList(res);
                Ok(self.get_cached_files())
            },
            Err(err) => {
                match err {
                    Error::ResponseError(c) => Err(UiActions::ShowNotification(c.content, NotificationType::Error)),
                    Error::Serde(_) => { // Null response
                        self.cached_files = FilesList::default();
                        Ok(self.get_cached_files())
                    }, 
                    _ => Err(UiActions::ShowNotification(err.to_string(), NotificationType::Error))
                }
            }
        }
    }

    /// Go to next folder, and return files list
    pub async fn next(&mut self, dir_name: &str) -> Result<Vec<FilesListInner>, UiActions> {
        self.change_dir(self.active_dir.join(Path::new(dir_name))).await
    }

    // Go to previous folder, and return files list
    pub async fn prev(&mut self) -> Result<Vec<FilesListInner>, UiActions> {
        let mut target = self.active_dir.clone();
        target.pop();
        self.change_dir(target).await
    }

    pub async fn make_dir(&mut self, target_dir: &str) -> Result<(), UiActions> {
        match files_make_directory(&self.cfg.api_conf, format_dir_for_req(&self.active_dir.join(Path::new(target_dir)).to_str().unwrap()).as_str()).await {
            Ok(_) => {
                self.cached_files.0.push(FilesListInner { name: target_dir.to_owned(), is_dir: Some(true), size: None, mod_time: 0 });
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
}

impl super::Service for FileManager {
    fn update_config_from_app(&mut self, app_cfg: crate::config::app::ApplicationConfig) {
        let server_api_conf = app_cfg.server_api_config();

        self.cfg.api_conf.base_path = server_api_conf.base_path().to_owned();
        self.cfg.api_conf.bearer_access_token = Some(server_api_conf.jwt().to_owned());
    }
}