use {
    reqwest::Client,
    crate::config::files::FilesServerConfig
};

pub struct FileManager {
    client: Client,
    cfg: FilesServerConfig
}

impl FileManager {
    fn cloned_client(&self) -> Client {
        self.client.clone()
    }

    pub fn new(client: Client, cfg: Option<FilesServerConfig>) -> Self {
        let cfg = match cfg {
            Some(x) => x,
            None => FilesServerConfig::default(),
        };
        Self { client: client.clone(), cfg }
    }

    pub fn config(&self) -> &FilesServerConfig {
        &self.cfg
    }

    pub fn config_mut(&mut self) -> &mut FilesServerConfig {
        &mut self.cfg
    }
}

impl super::Service for FileManager {
    fn update_config_from_app(&mut self, app_cfg: crate::config::app::ApplicationConfig) {
        self.cfg.srv_com_mut().clone_from(app_cfg.server_com_config());
    }
}