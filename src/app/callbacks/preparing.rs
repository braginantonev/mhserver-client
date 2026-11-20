use {
    crate::{
        NotificationType, OS, PreparingInternal, PreparingSettings, PreparingStates, UpdateStatus, actions::{MainActions::self, PreparingActions, UiActions}, app::{
            Application, di::build_intercepted_channel, transport::{Channel, init_tls_config}
        }, config::app::save_pem_cert, service::{
            auth::extend::is_jwt_expired, preparing,
        }
    }, slint::ComponentHandle, tonic::transport::{ClientTlsConfig, Endpoint, Error},
};

#[inline]
async fn try_connect_to_server(address: String, tls_config: ClientTlsConfig) -> Result<tonic::transport::Channel, Error> {
    Endpoint::from_shared(address)?
        .tls_config(tls_config)?
        .connect_timeout(std::time::Duration::from_secs(5))
        .connect()
        .await
}

impl Application {
    pub fn init_preparing(&self) {
        let win_weak = self.ui_window.as_weak();
        let preparing_internal = self.ui_window.global::<PreparingInternal>();

        #[cfg(target_os = "linux")]
        self.ui_window
            .global::<PreparingSettings>()
            .set_current_os(OS::Linux);

        #[cfg(target_os = "windows")]
        self.ui_window
            .global::<PreparingSettings>()
            .set_current_os(OS::Windows);

        preparing_internal.on_handle_state({
            let win = win_weak.clone();
            let cfg = self.cfg.clone();
            let tls_cfg = self.tls_cfg.clone();
            let chan = self.chan.clone();

            move |state: PreparingStates| {
                let win = win.clone();
                let cfg = cfg.clone();
                let tls_cfg = tls_cfg.clone();
                let chan = chan.clone();

                tokio::spawn(async move {
                    let need_user_action: bool = match state {
                        PreparingStates::Greeting => return, // greeting will invoke next for self

                        PreparingStates::Connection => {
                            let address = String::from(cfg.read().await.server_api_config().base_path());
                            let tls_cfg = tls_cfg.lock().await.clone();

                            if let Some(tls_cfg) = tls_cfg {
                                address.is_empty() ||  match try_connect_to_server(address, tls_cfg.clone()).await {
                                    Ok(c) => {
                                        *chan.write().await = Some(Channel::new(c.clone(), build_intercepted_channel(c, "")));
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
                            } else {
                                MainActions::ShowNotification(
                                    "Failed init TLS".to_owned(),
                                    "TLS server certificate not found".to_owned(),
                                    NotificationType::Error
                                ).run_in_event_loop(win.clone());
                                true
                            }
                        },

                        PreparingStates::Update => match preparing::check_update().await {
                            Ok(st ) => match st {
                                UpdateStatus::Available | UpdateStatus::Required => true,
                                _ => false
                            },
                            Err(err) => {
                                MainActions::from(err).run_in_event_loop(win.clone());
                                false
                            }
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

                        PreparingStates::End => {
                            let mut lock = chan.write().await;
                            let inner_chan = lock.as_ref().unwrap().inner();
                            let jwt = cfg.read().await.server_api_config().jwt().to_owned();
                            *lock = Some(Channel::new(inner_chan.clone(), build_intercepted_channel(inner_chan, &jwt)));
                            false
                        },
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
            let tls_cfg = self.tls_cfg.clone();
            let chan = self.chan.clone();

            move |srv_addr, tls_cert_filepath| {
                let win = win.clone();
                let cfg = cfg.clone();
                let tls_cfg = tls_cfg.clone();
                let chan = chan.clone();

                tokio::spawn(async move {
                    let tls_cfg = {
                        let mut lock = tls_cfg.lock().await;

                        if lock.is_none() && tls_cert_filepath.is_empty() {
                            MainActions::ShowNotification(
                                "TLS certificate not set".to_owned(),
                                "Old TLS not found and new is not set".to_owned(),
                                NotificationType::Error
                            ).run_in_event_loop(win);
                            return
                        }

                        // mv new cert to config path
                        if !tls_cert_filepath.is_empty() {
                            if let Err(err) = save_pem_cert(&tls_cert_filepath, true) {
                                MainActions::ShowNotification(
                                    "Failed save new TLS cert".to_owned(),
                                    err.to_string(),
                                    NotificationType::Error
                                ).run_in_event_loop(win);
                                return
                            }
                        }

                        match init_tls_config() {
                            Ok(cfg) => {
                                *lock = Some(cfg.clone());
                                cfg
                            },
                            Err(err) => {
                                MainActions::ShowNotification(
                                    "Failed init tls config".to_owned(),
                                    err.to_string(),
                                    NotificationType::Error
                                ).run_in_event_loop(win);
                                return
                            }
                        }
                    };
                    
                    match try_connect_to_server(srv_addr.to_string(), tls_cfg).await {
                        Ok(c) => {
                            cfg.write()
                                .await
                                .server_api_config_mut()
                                .set_base_path(&srv_addr);
                            *chan.write().await = Some(Channel::new(c.clone(), build_intercepted_channel(c, "")));
                            PreparingActions::InvokeNextState.run_in_event_loop(win);
                        }
                        Err(err) => {
                            MainActions::ShowNotification(
                                "Failed connect to server".to_owned(),
                                err.to_string(),
                                NotificationType::Error,
                            )
                            .run_in_event_loop(win);
                        }
                    };
                });
            }
        });

        preparing_internal.on_update_windows({
            let win = win_weak.clone();

            move || {
                let win = win.clone();

                tokio::spawn(async move {
                    match preparing::download_update().await {
                        Ok(_) => MainActions::ShowNotification("update downloaded".to_owned(), String::default(), NotificationType::Info,).run_in_event_loop(win.clone()),
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
