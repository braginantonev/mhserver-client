use {
    crate::{
        NotificationType, 
        actions::UiActions, 
        config::files::FileServiceConfig
    }, 
    openapi::{
        apis::{ default_api::get_files_list, Error },
        models::FilesListInner
    }, 
    std::{
        path::{Path, PathBuf},
        str::FromStr
    }
};

fn back_dir() -> FilesListInner {
    FilesListInner { name: "..".to_owned(), is_dir: Some(true), size: None, mod_time: 0 }
}

fn add_back_to_files(files: Vec<FilesListInner>) -> Vec<FilesListInner> {
    let mut with_back = vec![back_dir()];
    with_back.extend(files.into_iter());
    with_back
}

pub struct FileManager {
    cfg: FileServiceConfig,
    active_dir: PathBuf
}

impl FileManager {
    async fn get_files_from(&self, target_dir: &str) -> Result<Vec<FilesListInner>, UiActions> {
        match get_files_list(&self.cfg.api_conf, target_dir).await {
            Ok(res ) => if target_dir != "/" { Ok(add_back_to_files(res)) } else { Ok(res) }
            Err(err) => {
                match err {
                    Error::ResponseError(c) => Err(UiActions::ShowNotification(c.content, NotificationType::Error)),
                    Error::Serde(_) => Ok(vec![back_dir()]), // Null response
                    _ => Err(UiActions::ShowNotification(err.to_string(), NotificationType::Error))
                }
            }
        }
    }

    /// Change file manager active directory
    async fn change_dir(&mut self, target_dir: PathBuf) -> Result<Vec<FilesListInner>, UiActions> {
        // I know this is not the best way to do it, but while I don't change server API, there is one way, which can I see.
        let mut send_dir = target_dir.to_str().unwrap_or("/").to_owned();

        // API required
        if !send_dir.ends_with("/") {
            send_dir += "/";
        }
        
        let files = self.get_files_from(send_dir.as_str()).await;

        if files.is_ok() {
            self.active_dir = target_dir;
        }

        files
    }

    pub fn new(cfg: FileServiceConfig) -> Self {
        Self { 
            cfg,
            active_dir: PathBuf::from_str("/").unwrap()
        }
    }

    pub fn get_current_dir(&self) -> &str {
        self.active_dir.to_str().unwrap_or("/")
    }

    /// Return vector of file infos from active dir
    pub async fn get_files(&self) -> Result<Vec<FilesListInner>, UiActions> {
        self.get_files_from(self.active_dir.to_str().unwrap()).await
    }

    /// This is similar change_dir(), which use a current active dir as root
    pub async fn next(&mut self, dir_name: &str) -> Result<Vec<FilesListInner>, UiActions> {
        self.change_dir(self.active_dir.join(Path::new(dir_name))).await
    }

    pub async fn prev(&mut self) -> Result<Vec<FilesListInner>, UiActions> {
        let mut target = self.active_dir.clone();
        target.pop();
        self.change_dir(target).await
    }

    
}

impl super::Service for FileManager {
    fn update_config_from_app(&mut self, app_cfg: crate::config::app::ApplicationConfig) {
        let server_api_conf = app_cfg.server_api_config();

        self.cfg.api_conf.base_path = server_api_conf.base_path().to_owned();
        self.cfg.api_conf.bearer_access_token = Some(server_api_conf.jwt().to_owned());
    }
}