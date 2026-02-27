use {
    crate::{NotificationType, actions::UiActions, config::files::FileServiceConfig}, api::data, reqwest::Client, std::{path::{Path, PathBuf}, str::FromStr}
};

pub struct FileManager {
    client: Client,
    cfg: FileServiceConfig,
    active_dir: PathBuf
}

impl FileManager {
    fn cloned_client(&self) -> Client {
        self.client.clone()
    }

    pub fn new(client: Client, cfg: Option<FileServiceConfig>) -> Self {
        let cfg = match cfg {
            Some(x) => x,
            None => FileServiceConfig::default(),
        };
        Self { 
            client: client.clone(),
            cfg,
            active_dir: PathBuf::from_str("/").expect("failed create default file manager path")
        }
    }

    pub fn config(&self) -> &FileServiceConfig {
        &self.cfg
    }

    pub fn config_mut(&mut self) -> &mut FileServiceConfig {
        &mut self.cfg
    }

    async fn get_files_from(&self, target_dir: &str) -> Result<Vec<data::FileInfo>, UiActions> {
        match data::get_files_v1(
            api::AuthorizedRequest::new(self.cloned_client(), self.cfg.srv_com().server_address(), self.cfg.srv_com().user_jwt()),
            target_dir
        ).await {
            Ok(res ) => Ok(res),
            Err(err) => Err(UiActions::ShowNotification(err.to_string(), NotificationType::Error))
        }
    }

    /// Return vector of file infos from active dir
    pub async fn get_files(&self) -> Result<Vec<data::FileInfo>, UiActions> {
        self.get_files_from(self.active_dir.to_str().unwrap()).await
    }

    /// Change file manager active direction and return file infos
    pub async fn change_dir(&mut self, target_dir: PathBuf) -> Result<Vec<data::FileInfo>, UiActions> {
        let files = self.get_files_from(target_dir.to_str().unwrap()).await;

        if files.is_ok() {
            self.active_dir = target_dir;
        }

        files
    }

    /// This is similar change_dir(), which use a current active dir as root
    pub async fn change_dir_from_current(&mut self, next_dir: &Path) -> Result<Vec<data::FileInfo>, UiActions> {
        self.change_dir(self.active_dir.join(next_dir)).await
    }
}

impl super::Service for FileManager {
    fn update_config_from_app(&mut self, app_cfg: crate::config::app::ApplicationConfig) {
        self.cfg.srv_com_mut().clone_from(app_cfg.server_com_config());
    }
}