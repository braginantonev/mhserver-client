use {
    crate::{
        PreparingInternal, PreparingStates, State, app::Application, service::*
    }, api::apis::configuration::Configuration, slint::ComponentHandle, std::sync::Arc, tokio::sync::RwLock,
};

async fn login_expired() -> bool {
    true // tmp
}

impl Application {
    pub fn init(&self) {
        let win_weak = self.ui_window.as_weak();       

        // Services
        let files_service = Arc::new(RwLock::new(files::FileManager::new()));
        let services: Vec<Arc<RwLock<dyn Service + Send + Sync>>> = vec![files_service.clone()];

        //* Important preparing callbacks

        let preparing_internal = self.ui_window.global::<PreparingInternal>();

        preparing_internal.on_handle_state({
            let win = win_weak.clone();
            let cfg = self.cfg.clone();
            let client = self.http_client.clone();
            move || {
                let win = win.clone();
                let cfg = cfg.clone();
                let client = client.clone();
                let _ = slint::spawn_local(async move {
                    let mut api_cfg = Configuration::new();
                    api_cfg.client = client;

                    {
                        let lock = cfg.read().await;
                        api_cfg.base_path = lock.server_api_config().base_path().to_owned();
                        api_cfg.bearer_access_token = Some(lock.server_api_config().jwt().to_owned());
                    }

                    let win = win.upgrade().unwrap();
                    
                    if match win.global::<State>().get_preparing() {
                        PreparingStates::Greeting => return,
                        PreparingStates::Connection => !preparing::ping(&api_cfg).await,
                        PreparingStates::Login => login_expired().await,
                        PreparingStates::Register => false,
                        PreparingStates::End => false,
                    } {
                        win.global::<PreparingInternal>().set_prepare_needed(true);
                    } else {
                        win.global::<State>().invoke_next();
                    }
                });  
            }
        });
        
        preparing_internal.on_update_services({
            let update_cfg = self.cfg.clone();
            move || {
                let update_cfg = update_cfg.clone();
                let services = services.clone();
                tokio::spawn(async move {
                    let update_cfg = update_cfg.read().await;
                    for s in services {
                        s.write().await.update_config(update_cfg.clone());
                    }
                });
            }
        });

        self.init_preparing_callbacks();
    }
}