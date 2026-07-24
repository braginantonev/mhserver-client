use {
    crate::{
        MainWindow, NotificationType, PreparingInternal, PreparingStates, State, actions::UiActions, app::Application, config::app::ApplicationConfig, service::*
    }, api::apis::configuration::Configuration, slint::{ComponentHandle, Weak}, std::sync::Arc, tokio::sync::RwLock,
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
                        PreparingStates::Connection => !preparing::ping(&api_cfg, None).await,
                        PreparingStates::Login => true,
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

        /*self.ui_window.on_change_preparing_state({
            let win = win_weak.clone();
            let cfg = self.cfg.clone();
            let tools = tools_service.clone();

            move |new_preparing_state| {
                let win = win.clone();
                let cfg = cfg.clone();
                let tools = tools.clone();

                tokio::spawn(async move {
                    println!("go to preparing {:?}", new_preparing_state);
                    match new_preparing_state {
                        PreparingStates::Normal => {
                            UiActions::ChangePreparingState(new_preparing_state.next())
                        },
                        PreparingStates::CheckConn => {
                            let api_cfg = cfg.read().await;

                            UiActions::ChangePreparingState(if tools.read().await.ping(Some(api_cfg.server_api_config().base_path())).await {
                                new_preparing_state.next()
                            } else {
                                PreparingStates::Connection
                            })
                        },
                        PreparingStates::CheckAuth => {
                            UiActions::ChangePreparingState(if cfg.read().await.server_api_config().jwt() == "" {
                                PreparingStates::Login
                            } else {
                                new_preparing_state.next()
                            })
                        },
                        PreparingStates::End => {
                            cfg.read().await.save_to_file();

                            //Todo: add multiply events update
                            let _ = win.upgrade_in_event_loop(|win| {
                                win.invoke_update_service_configs();
                            });

                            println!("end preparing");

                            UiActions::ChangeAppState(crate::AppStates::Main)
                        }
                        _ => UiActions::ShowNotification(format!("unexpected preparing state: {:?}", new_preparing_state), NotificationType::Info)
                    }.run_in_event_loop(win);
                });
            }
        });

        self.ui_window.on_connect({
            let win = win_weak.clone();
            let cfg = self.cfg.clone();
            let tools = tools_service.clone();

            move |srv_addr| {
                let win = win.clone();
                let cfg = cfg.clone();
                let tools = tools.clone();

                tokio::spawn(async move {
                    if tools.read().await.ping(Some(srv_addr.as_str())).await {
                        cfg.write().await.server_api_config_mut().set_base_path(srv_addr.as_str());

                        //Todo: add multiply events update
                        let _ = win.upgrade_in_event_loop(|win| {
                            win.invoke_update_service_configs();
                        });
                        
                        UiActions::ChangePreparingState(PreparingStates::Connection.next())
                    } else {
                        UiActions::ShowNotification("Server is off or unavailable".to_owned(), NotificationType::Error)
                    }.run_in_event_loop(win);
                });
            }
        });

        self.ui_window.on_update_service_configs({
            let main_cfg = self.cfg.clone();
            let services = self.services.clone();

            move || {
                let main_cfg = main_cfg.clone();
                let services = services.clone();

                for service in services {
                    let cfg = main_cfg.clone();

                    tokio::spawn(async move {
                        let cfg = cfg.read().await.clone();
                        service.write().await.update_config_from_app(cfg);
                    });
                }
            }
        });*/
    }
}