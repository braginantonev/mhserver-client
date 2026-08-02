pub mod errors;
pub mod callbacks;
mod init;

use {
    crate::{MainWindow, config::app::ApplicationConfig}, 
    errors::{ApplicationError, ApplicationErrors}, 
    slint::ComponentHandle, std::sync::Arc, tokio::sync::RwLock
};

pub struct Application {
    ui_window: MainWindow,
    base_client: reqwest::Client,
    cfg: Arc<RwLock<ApplicationConfig>>,
}

impl Application {
    pub fn new() -> Result<Self, ApplicationError> {
        let win = match MainWindow::new() {
            Ok(win) => win,
            Err(err) => return Err(ApplicationError::new(ApplicationErrors::FailedCreateWindow(err.to_string()))),
        };

        let base_client = match reqwest::Client::builder()
            .tls_info(true)
            .tls_backend_rustls()
            .danger_accept_invalid_certs(true)
            .timeout(std::time::Duration::new(2, 0))
            .build() {
                Ok(cl) => cl,
                Err(err) => return Err(ApplicationError::new(ApplicationErrors::FailedCreateHttpClient(err.to_string())))
            };

        let cfg = Arc::new(RwLock::new(match ApplicationConfig::from_file() {
            Ok(res) => res,
            Err(_) => ApplicationConfig::default()
        }));

        let s = Self {
            ui_window: win,
            base_client,
            cfg,
        };

        s.init();

        Ok(s)
    }

    pub async fn run(&self) -> Result<(), ApplicationError> {
        let res = self.ui_window.run();
        self.cfg.read().await.save_to_file();
        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(ApplicationError::new(ApplicationErrors::WindowError(err.to_string())))
        }
    }
}