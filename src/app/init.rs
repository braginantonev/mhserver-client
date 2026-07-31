use {
    crate::{
        PreparingInternal, PreparingStates, UpdateStatus, State, actions::{MainActions, UiActions}, app::Application, service::*
    }, api::apis::configuration::Configuration, slint::ComponentHandle, std::sync::Arc, tokio::sync::RwLock,
};

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
                    
                    // check need preparing or not
                    if match win.global::<State>().get_preparing() {
                        PreparingStates::Greeting => return,
                        PreparingStates::Connection => preparing::ping(&api_cfg).await.is_err(),
                        PreparingStates::Update => match preparing::update_status(&api_cfg).await {
                            Ok(st ) => match st {
                                UpdateStatus::Available => true,
                                UpdateStatus::CantCheck => {
                                    MainActions::ShowNotification("failed check update".to_owned(), String::default(), crate::NotificationType::Info).run(win.clone_strong());
                                    false
                                },
                                _ => false
                            },
                            Err(err) => {
                                MainActions::from(err).run(win.clone_strong());
                                false
                            }
                        },
                        PreparingStates::Login => api_cfg.bearer_access_token.unwrap_or_default().is_empty(),
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
            let client = self.http_client.clone();
            move || {
                let update_cfg = update_cfg.clone();
                let services = services.clone();
                let client = client.clone();
                tokio::spawn(async move {
                    let update_cfg = update_cfg.read().await;
                    for s in services {
                        s.write().await.update_config(client.clone(), update_cfg.clone());
                    }
                });
            }
        });

        self.init_preparing_callbacks();
        self.init_files_callbacks(files_service);
        self.init_window_callbacks();
    }
}