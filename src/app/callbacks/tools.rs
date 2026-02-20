use {
    crate::{
        NotificationType, PreparingStates, 
        actions::UiActions, 
        app::{Application, ApplicationConfig},
    },
    slint::ComponentHandle, 
    std::sync::Arc,
    tokio::sync::RwLock,
};


impl Application {
    pub fn init_preparing_callbacks(&self, preparing_cfg: Arc<RwLock<ApplicationConfig>>) {
        let win_weak = self.ui_window.as_weak();

        self.ui_window.on_change_preparing_state({
            let win = win_weak.clone();
            let main_cfg = self.cfg.clone();
            let pre_cfg = preparing_cfg.clone();
            let client = self.http_client.clone();

            move |new_preparing_state| {
                let win = win.clone();
                let main_cfg = main_cfg.clone();
                let pre_cfg = pre_cfg.clone();
                let client = client.clone();

                tokio::spawn(async move {
                    println!("go to preparing {:?}", new_preparing_state);
                    match new_preparing_state {
                        PreparingStates::Normal => {
                            UiActions::ChangePreparingState(new_preparing_state.next())
                        },
                        PreparingStates::CheckConn => {
                            UiActions::ChangePreparingState(match api::ping::ping(client, pre_cfg.read().await.server_com_config().server_address()).await {
                                Ok(_) => new_preparing_state.next(),
                                Err(_) => PreparingStates::Connection
                            })
                        },
                        PreparingStates::CheckAuth => {
                            UiActions::ChangePreparingState(if pre_cfg.read().await.server_com_config().user_jwt() == "" {
                                PreparingStates::Login
                            } else {
                                new_preparing_state.next()
                            })
                        }
                        PreparingStates::End => {
                            let prepared_cfg = pre_cfg.read().await.clone();

                            main_cfg.write().await.update_from_self(prepared_cfg);
                            main_cfg.read().await.save_to_file();

                            let _ = win.upgrade_in_event_loop(|win| {
                                win.invoke_update_services();
                            });

                            println!("end preparing");

                            UiActions::ChangeAppState(crate::AppStates::Main)
                        }
                        _ => UiActions::ShowNotification(format!("unexpected preparing state: {:?}", new_preparing_state), NotificationType::Info)
                    }.run_in_event_loop(win);
                });
            }
        });

        self.ui_window.on_update_services({
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

        self.ui_window.on_connect({
            let win = win_weak.clone();
            let client = self.http_client.clone();
            let pre_cfg = preparing_cfg.clone();

            move |srv_addr| {
                let win = win.clone();
                let client = client.clone();
                let pre_cfg = pre_cfg.clone();

                tokio::spawn(async move {
                    match api::ping::ping(client, srv_addr.as_str()).await {
                        Ok(_) => {
                            pre_cfg.write().await.server_com_config_mut().set_server_address(srv_addr.as_str());
                            UiActions::ChangePreparingState(PreparingStates::Connection.next())
                        },
                        Err(desc) => UiActions::ShowNotification(desc.to_string(), NotificationType::Error)
                    }.run_in_event_loop(win);
                });
            }
        });
    }
}