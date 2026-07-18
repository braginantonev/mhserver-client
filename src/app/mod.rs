pub mod errors;
pub mod callbacks;

use {
    crate::{
        MainWindow, 
        config::{self, app::ApplicationConfig}, 
        service::*
    }, errors::{ApplicationError, ApplicationErrors}, slint::ComponentHandle, std::sync::Arc, tokio::sync::RwLock
};

pub struct Application {
    ui_window: MainWindow,
    http_client: reqwest::Client,
    cfg: Arc<RwLock<ApplicationConfig>>,
    services: Vec<Arc<RwLock<dyn Service + Send + Sync>>>
}

impl Application {
    fn add_service<T: Service + Send + Sync + 'static>(&mut self, service: Arc<RwLock<T>>) {
        self.services.push(service);
    }

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

        let cfg = Arc::new(RwLock::new(match ApplicationConfig::from_file() {
            Ok(res) => res,
            Err(_) => ApplicationConfig::default()
        }));

        Ok(Self{
            ui_window: win,
            http_client,
            cfg,
            services: Vec::new()
        })
    }

    pub async fn run(&mut self) -> Result<(), ApplicationError> {
        let mut api_conf = api::apis::configuration::Configuration::new();
        api_conf.client = self.http_client.clone();

        {
            let lock = self.cfg.read().await;
            let server_api_conf = lock.server_api_config();

            api_conf.base_path = server_api_conf.base_path().to_owned();
            api_conf.bearer_access_token = Some(server_api_conf.jwt().to_owned());
        }

        let tools_service = Arc::new(RwLock::new(tools::ServerTools::new(api_conf.clone())));
        self.add_service(tools_service.clone());

        let auth_service = Arc::new(RwLock::new(auth::Authenticator::new(api_conf.clone())));
        self.add_service(auth_service.clone());

        let files_service = Arc::new(RwLock::new(files::FileManager::new(config::files::FileServiceConfig::new(api_conf.clone()))));
        self.add_service(files_service.clone());

        self.init_preparing_callbacks(tools_service.clone());
        self.init_auth_callbacks(auth_service);
        self.init_files_callbacks(files_service);

        match self.ui_window.run() {
            Ok(_) => Ok(()),
            Err(err) => Err(ApplicationError::new(ApplicationErrors::WindowError(err.to_string())))
        }
    }
}