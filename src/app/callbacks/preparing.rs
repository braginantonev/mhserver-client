use {
    crate::{
        NotificationType, OS, PreparingInternal, PreparingSettings, PreparingStates, actions::{MainActions, PreparingActions, UiActions}, app::Application, service::{auth::extend::is_jwt_expired}
    }, slint::ComponentHandle, tonic::transport::{Channel, Endpoint, Error},
};

#[inline]
async fn try_connect_to_server(address: String) -> Result<Channel, Error> {
    return Endpoint::from_shared(address)?.connect_timeout(std::time::Duration::from_secs(5)).connect().await
}

impl Application {
    pub fn init_preparing(&self) {
        let win_weak = self.ui_window.as_weak();
        let preparing_internal = self.ui_window.global::<PreparingInternal>();

        #[cfg(target_os = "linux")]
        self.ui_window.global::<PreparingSettings>().set_current_os(OS::Linux);

        #[cfg(target_os = "windows")]
        self.ui_window.global::<PreparingSettings>().set_current_os(OS::Windows);

        preparing_internal.on_handle_state({
            let win = win_weak.clone();
            let cfg = self.cfg.clone();
            let chan = self.chan.clone();

            move |state: PreparingStates| {
                let win = win.clone();
                let cfg = cfg.clone();
                let chan = chan.clone();

                tokio::spawn(async move {
                    let need_user_action: bool = match state {
                        PreparingStates::Greeting => return, // greeting will invoke next for self

                        PreparingStates::Connection => {
                            if chan.get().is_some() {
                                return MainActions::ShowNotification(
                                    "Failed connect to server".to_owned(), 
                                    "Server is off or address was changed\nPlease restart application".to_owned(),
                                    NotificationType::Error
                                ).run_in_event_loop(win);
                            }

                            let address = String::from(cfg.read().await.server_api_config().base_path());

                            address.is_empty() || match try_connect_to_server(address).await {
                                Ok(c) => {
                                    chan.set(c);
                                    false
                                },
                                Err(_) => {
                                    MainActions::ShowNotification(
                                        "Failed connect to server".to_owned(), 
                                        "Server is off or address was changed".to_owned(), 
                                        NotificationType::Error
                                    ).run_in_event_loop(win.clone());
                                    true
                                },
                            }
                        },

                        PreparingStates::Update => /*match preparing::update_status(&api_cfg).await*/ {
                            /*
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
                            }*/

                            // temporary dropped. fix in 3.x
                            false
                        },

                        PreparingStates::Login => {
                            let api_cfg = cfg.read().await.server_api_config().clone();

                            api_cfg.jwt().is_empty() || match is_jwt_expired(api_cfg.jwt()) {
                                Ok(b) => b,
                                Err(err) => {
                                    MainActions::ShowNotification("Failed check auth expiration".to_owned(), err.to_string(), NotificationType::Error).run_in_event_loop(win.clone());
                                    true
                                },
                            }
                        },

                        PreparingStates::Register => false,
                        PreparingStates::End => false,
                    };

                    if need_user_action {
                        PreparingActions::InvokePreparing.run_in_event_loop(win);
                    } else {
                        PreparingActions::InvokeNextState.run_in_event_loop(win);
                    }
                });  
            }
        });

        preparing_internal.on_connect({
            let win = win_weak.clone();
            let cfg = self.cfg.clone();
            let chan = self.chan.clone();

            move |srv_addr| {
                let win = win.clone();
                let cfg = cfg.clone();
                let chan = chan.clone();

                tokio::spawn(async move {
                    if chan.get().is_some() {
                        return MainActions::ShowNotification(
                            "Failed connect to server".to_owned(), 
                            "Server is off or address was changed\nPlease restart application".to_owned(),
                            NotificationType::Error
                        ).run_in_event_loop(win);
                    }

                    match try_connect_to_server(srv_addr.to_string()).await {
                        Ok(c) => {
                            cfg.write().await.server_api_config_mut().set_base_path(&srv_addr);
                            chan.set(c); 
                        },
                        Err(err) => {
                            MainActions::ShowNotification(
                                "Failed connect to server".to_owned(), 
                                err.to_string(), 
                                NotificationType::Error
                            ).run_in_event_loop(win);
                        },
                    };
                });
            }
        });

        #[cfg(target_os = "windows")]
        preparing_internal.on_update_windows({
            let win = win_weak.clone();
            let http_client = self.base_client.clone();

            move || {
                let win = win.clone();
                let http_client = http_client.clone();

                tokio::spawn(async move {
                    match preparing::download_update(&api_cfg(http_client, "".to_owned())).await {
                        Ok(_) => MainActions::ShowNotification("update downloaded".to_owned(), String::default(), NotificationType::Info).run_in_event_loop(win.clone()),
                        Err(err) => {
                            MainActions::from(err).run_in_event_loop(win);
                            return;
                        }
                    };
                    
                    std::process::Command::new("cmd")
                        .args(&["/c", "start", "/b", "update_windows.bat"])
                        .spawn()
                        .expect("Failed to start update");
                    
                    MainActions::ExitApp.run_in_event_loop(win);
                });
            }
        });
    }
}
