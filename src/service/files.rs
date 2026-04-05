use {
    crate::{
        NotificationType, 
        actions::UiActions, 
        config::files::FileServiceConfig
    }, 
    openapi::{
        apis::default_api::get_files_list,
        models::FilesListInner
    }, 
    std::{
        path::{Path, PathBuf},
        str::FromStr
    }
};

pub struct FileManager {
    cfg: FileServiceConfig,
    active_dir: PathBuf
}

impl FileManager {
    pub fn new(cfg: FileServiceConfig) -> Self {
        Self { 
            cfg,
            active_dir: PathBuf::from_str("/").unwrap()
        }
    }

    async fn get_files_from(&self, target_dir: &str) -> Result<Vec<FilesListInner>, UiActions> {
        match get_files_list(&self.cfg.api_conf, target_dir).await {
            Ok(res ) => Ok(res),
            Err(err) => {
                if let openapi::apis::Error::ResponseError(x) = err {
                    Err(UiActions::ShowNotification(x.content, NotificationType::Error))
                } else {
                    Err(UiActions::ShowNotification(err.to_string(), NotificationType::Error))
                }
            }
        }
    }

    pub fn get_current_dir(&self) -> &str {
        self.active_dir.to_str().unwrap_or("/")
    }

    /// Return vector of file infos from active dir
    pub async fn get_files(&self) -> Result<Vec<FilesListInner>, UiActions> {
        self.get_files_from(self.active_dir.to_str().unwrap()).await
    }

    /// Change file manager active directory
    pub async fn change_dir(&mut self, target_dir: PathBuf) -> Result<Vec<FilesListInner>, UiActions> {
        // I know this is not the best way to do it, but while I don't change server API, there is one way, which can I see.
        let send_dir = target_dir.to_str().unwrap_or("/").to_owned() + "/";
        
        let files = self.get_files_from(send_dir.as_str()).await;

        if files.is_ok() {
            self.active_dir = target_dir;
        }

        files
    }

    /// This is similar change_dir(), which use a current active dir as root
    pub async fn change_dir_from_current(&mut self, next_dir: &Path) -> Result<Vec<FilesListInner>, UiActions> {
        self.change_dir(self.active_dir.join(next_dir)).await
    }
}

impl super::Service for FileManager {
    fn update_config_from_app(&mut self, app_cfg: crate::config::app::ApplicationConfig) {
        let server_api_conf = app_cfg.server_api_config();

        self.cfg.api_conf.base_path = server_api_conf.base_path().to_owned();
        self.cfg.api_conf.bearer_access_token = Some(server_api_conf.jwt().to_owned());
    }
}