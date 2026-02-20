use {
    reqwest::Client,
    crate::config::files::FileServiceConfig
};

pub struct FileManager {
    client: Client,
    cfg: FileServiceConfig
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
        Self { client: client.clone(), cfg }
    }

    pub fn config(&self) -> &FileServiceConfig {
        &self.cfg
    }

    pub fn config_mut(&mut self) -> &mut FileServiceConfig {
        &mut self.cfg
    }
}

impl super::Service for FileManager {
    fn update_config_from_app(&mut self, app_cfg: crate::config::app::ApplicationConfig) {
        self.cfg.srv_com_mut().clone_from(app_cfg.server_com_config());
    }
}