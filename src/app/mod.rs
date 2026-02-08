pub mod errors;
pub mod callbacks;

use {
    crate::{
        MainWindow, NotificationType, PreparingStates, actions::UiActions, config::app::ApplicationConfig, service::*
    },
    errors::{ApplicationError, ApplicationErrors},
    slint::ComponentHandle, 
    std::sync::Arc,
    tokio::sync::OnceCell,
};

pub struct Application {
    ui_window: MainWindow,
    http_client: reqwest::Client,
    cfg: Arc<OnceCell<ApplicationConfig>>
}

impl Application {
    pub fn new() -> Result<Self, ApplicationError> {
        let win = match MainWindow::new() {
            Ok(win) => win,
            Err(err) => return Err(ApplicationError::new(ApplicationErrors::FailedCreateWindow(err.to_string()))),
        };

        let http_client = match reqwest::Client::builder()
            .tls_info(true)
            .tls_backend_rustls()
            .danger_accept_invalid_certs(true)
            .timeout(std::time::Duration::new(2, 0))
            .build() {
                Ok(cl) => cl,
                Err(err) => return Err(ApplicationError::new(ApplicationErrors::FailedCreateHttpClient(err.to_string())))
            };

        Ok(Self{
            ui_window: win,
            http_client: http_client,
            cfg: Arc::new(OnceCell::new()) 
        })
    }

    pub fn run(&mut self) -> Result<(), ApplicationError> {
        let preparing_cfg = Arc::new(tokio::sync::RwLock::new(match ApplicationConfig::from_file() {
            Ok(res) => res,
            Err(_) => {
                let buff = ApplicationConfig::default();
                buff.save_to_file();
                buff
            }
        }));

        let auth_service = Arc::new(auth::Authenticator::new(self.http_client.clone()));

        self.init_preparing_callbacks(preparing_cfg.clone());
        self.init_auth_callbacks(preparing_cfg.clone(), auth_service);

        match self.ui_window.run() {
            Ok(_) => Ok(()),
            Err(err) => Err(ApplicationError::new(ApplicationErrors::WindowError(err.to_string())))
        }
    }
}