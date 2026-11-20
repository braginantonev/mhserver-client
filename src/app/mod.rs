pub mod errors;
mod callbacks;
mod di;
mod transport;

use {
    crate::{MainWindow, config::app::ApplicationConfig}, errors::ApplicationError, slint::ComponentHandle, std::sync::Arc, tokio::sync::{Mutex, RwLock}, tonic::transport::ClientTlsConfig,
};

pub struct Application {
    ui_window: MainWindow,
    chan: Arc<RwLock<Option<transport::Channel>>>,
    cfg: Arc<RwLock<ApplicationConfig>>,

    // in not app cfg, because `ApplicationConfig` - will set from file
    // which `ClientTlsConfig` is not  
    tls_cfg: Arc<Mutex<Option<ClientTlsConfig>>>,
}

impl Application {
    pub fn new() -> Result<Self, ApplicationError> {
        let win = MainWindow::new().map_err(|x| ApplicationError::from(x))?; 

        let mut cfg = ApplicationConfig::from_file().unwrap_or_default();
        cfg.normalize();

        let tls_cfg = transport::init_tls_config().ok();

        let s = Self {
            ui_window: win,
            chan: Arc::default(),
            cfg: Arc::new(RwLock::new(cfg)),
            tls_cfg: Arc::new(Mutex::new(tls_cfg)),
        };

        Ok(s)
    }

    async fn init(&self) {
        self.init_apis();
        self.init_window_callbacks();
        self.init_preparing();
        self.init_services().await;
    }

    pub async fn run(&self) -> Result<(), ApplicationError> {
        self.init().await;
        self.ui_window.run().map_err(|x| ApplicationError::from(x))?;

        // Post processing
        self.cfg.read().await.save_to_file();

        Ok(())
    }
}
