pub mod errors;
mod callbacks;
mod di;

use {
    crate::{config::app::ApplicationConfig, MainWindow},
    errors::{ApplicationError, ApplicationErrors},
    slint::ComponentHandle,
    std::sync::Arc,
    tokio::sync::RwLock,
};

pub struct Application {
    ui_window: MainWindow,
    chan: Arc<RwLock<Option<tonic::transport::Channel>>>,
    cfg: Arc<RwLock<ApplicationConfig>>,
}

impl Application {
    pub fn new() -> Result<Self, ApplicationError> {
        let win = match MainWindow::new() {
            Ok(win) => win,
            Err(err) => {
                return Err(ApplicationError::new(
                    ApplicationErrors::FailedCreateWindow(err.to_string()),
                ))
            }
        };

        let cfg = Arc::new(RwLock::new(match ApplicationConfig::from_file() {
            Ok(res) => res,
            Err(_) => ApplicationConfig::default(),
        }));

        let s = Self {
            ui_window: win,
            chan: Arc::default(),
            cfg,
        };

        Ok(s)
    }

    async fn init(&self) {
        self.init_window_callbacks();
        self.init_preparing();
        self.init_services().await;
    }

    pub async fn run(&self) -> Result<(), ApplicationError> {
        self.init().await;
        let res = self.ui_window.run();

        self.cfg.read().await.save_to_file();
        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(ApplicationError::new(ApplicationErrors::WindowError(
                err.to_string(),
            ))),
        }
    }
}
