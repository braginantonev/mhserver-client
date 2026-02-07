pub mod errors;

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
    cfg: Arc<OnceCell<ApplicationConfig>>
}

impl Application {
    pub fn new() -> Result<Self, ApplicationError> {
        Ok(Self { cfg: Arc::new(OnceCell::new()) })
    }

    pub fn run(&mut self) -> Result<(), ApplicationError> {
        let main_window = match MainWindow::new() {
            Ok(win) => win,
            Err(err) => return Err(ApplicationError::new(ApplicationErrors::FailedCreateWindow(err.to_string()))),
        };

        let win_weak = main_window.as_weak();

        let http_client = match reqwest::Client::builder()
            .tls_info(true)
            .tls_backend_rustls()
            .danger_accept_invalid_certs(true)
            .timeout(std::time::Duration::new(2, 0))
            .build() {
                Ok(cl) => cl,
                Err(err) => return Err(ApplicationError::new(ApplicationErrors::FailedCreateHttpClient(err.to_string())))
            };
        
        //* Services

        let auth_service = Arc::new(auth::Authenticator::new(http_client.clone()));

        //* Window init

        let preparing_cfg = Arc::new(tokio::sync::RwLock::new(match ApplicationConfig::from_file() {
            Ok(res) => res,
            Err(_) => {
                let buff = ApplicationConfig::default();
                buff.save_to_file();
                buff
            }
        }));

        main_window.on_change_preparing_state({
            let win = win_weak.clone();
            let main_cfg = self.cfg.clone();
            let pre_cfg = preparing_cfg.clone();
            let client = http_client.clone();

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
                            if main_cfg.set(pre_cfg.read().await.clone()).is_err() {
                                panic!("failed init main config") // ! For debug only
                            };

                            main_cfg.get().unwrap().save_to_file();
                            println!("end of preparing");

                            UiActions::ChangeAppState(crate::AppStates::Main)
                        }
                        _ => UiActions::ShowNotification(format!("unexpected preparing state: {:?}", new_preparing_state), NotificationType::Info)
                    }.run_in_event_loop(win);
                });
            }
        });

        main_window.on_connect({
            let win = win_weak.clone();
            let client = http_client.clone();
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

        main_window.on_login({
            let win = win_weak.clone();
            let service = auth_service.clone();
            let pre_cfg = preparing_cfg.clone();

            move |username, password| {
                let win = win.clone();
                let service = service.clone();
                let pre_cfg = pre_cfg.clone();

                tokio::spawn(async move {
                    let (jwt, act) = service.login(
                        pre_cfg.read().await.server_com_config().server_address(),
                        api::auth::User::new(username.as_str(), password.as_str())
                    ).await;

                    if let Some(jwt) = jwt {
                        pre_cfg.write().await.server_com_config_mut().set_user_jwt(jwt.as_str());
                    }
                    act.run_in_event_loop(win);
                });
            }
        });

        main_window.on_register({
            let win = win_weak.clone();
            let service = auth_service.clone();
            let pre_cfg = preparing_cfg.clone();

            move |username, password, verify| {
                let win = win.clone();
                let service = service.clone();
                let pre_cfg = pre_cfg.clone();

                tokio::spawn(async move {
                    if password != verify {
                        UiActions::ShowNotification("Password and verify password not ident!".to_owned(), NotificationType::Error).run_in_event_loop(win);
                        return
                    }

                    service.register(
                        pre_cfg.read().await.server_com_config().server_address(),
                        api::auth::User::new(username.as_str(), password.as_str())
                    ).await.run_in_event_loop(win);
                });
            }
        });

        match main_window.run() {
            Ok(_) => Ok(()),
            Err(err) => Err(ApplicationError::new(ApplicationErrors::WindowError(err.to_string())))
        }
    }
}