use {
    crate::{
        NotificationType, PreparingStates, actions::UiActions, app::Application
    },
    slint::ComponentHandle, 
    std::sync::Arc,
    tokio::sync::RwLock,
};


impl Application {
    pub fn init_preparing_callbacks(&self, tools_service: Arc<RwLock<crate::service::tools::ServerTools>>) {
        let win_weak = self.ui_window.as_weak();

        self.ui_window.on_change_preparing_state({
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
        });
    }
}