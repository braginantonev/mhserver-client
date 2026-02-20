use {
    crate::{app::Application, service}, std::sync::Arc, tokio::sync::RwLock
};

impl Application {
    pub fn init_files_callbacks(&mut self, files_service: Arc<RwLock<service::files::FileManager>>) {
        self.ui_window.on_show_service({
            let service = files_service.clone();

            move |_| {
                let service = service.clone();
                tokio::spawn(async move {
                    println!("{:?}", service.read().await.config())
                });
            }
        });
    }
}