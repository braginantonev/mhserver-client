use {
    crate::{
        config::files::FileServiceConfig,
        service::*,
    },
    std::sync::Arc,
    tokio::sync::RwLock,
};

impl super::Application {
    pub async fn init_services(&self) {
        let app_cfg = self.cfg.read().await;

        // Files service
        let files_cfg = FileServiceConfig::new(app_cfg.download_dir());
        let files_service = Arc::new(RwLock::new(files::FileManager::new(files_cfg)));
        self.init_files_service(files_service);

        // Auth service
        self.init_auth();
    }
}