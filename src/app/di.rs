use {
    crate::{
        config::files::FileServiceConfig, service::{auth::Authenticator, *},
    }, std::sync::Arc, tokio::sync::RwLock,
};

impl super::Application {
    pub async fn init_services(&self) {
        let app_cfg = self.cfg.read().await;

        // Auth service
        let auth_service = Arc::new(RwLock::new(Authenticator::new())); // Arc and RwLock mb for future
        self.init_auth(auth_service);

        // Files service
        let files_cfg = FileServiceConfig::new(app_cfg.download_dir());
        let files_service = Arc::new(RwLock::new(files::FileManager::new(files_cfg)));
        self.init_files_service(files_service);
    }
}