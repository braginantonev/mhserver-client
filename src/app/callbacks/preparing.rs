use {
    crate::{
        NotificationType, PreparingInternal, actions::{MainActions, PreparingActions, UiActions}, app::Application, service::preparing
    }, api::apis::configuration::Configuration, reqwest::Client, slint::ComponentHandle
};

fn api_cfg(client: Client, base_path: String) -> Configuration {
    let mut cfg = Configuration::new();
    cfg.client = client;
    cfg.base_path = base_path;
    cfg
}

impl Application {
    pub fn init_preparing_callbacks(&self) {
        let win_weak = self.ui_window.as_weak();

        let preparing_internal = self.ui_window.global::<PreparingInternal>();

        preparing_internal.on_connect({
            let win = win_weak.clone();
            let cfg = self.cfg.clone();
            let http_client = self.http_client.clone();

            move |srv_addr| {
                let win = win.clone();
                let cfg = cfg.clone();
                let http_client = http_client.clone();

                tokio::spawn(async move {
                    match preparing::ping(&api_cfg(http_client, srv_addr.to_string())).await {
                        Ok(_) => {
                            cfg.write().await.server_api_config_mut().set_base_path(srv_addr.as_str());
                            PreparingActions::InvokeNextState.run_in_event_loop(win);
                        },
                        Err(err) => MainActions::from(err).run_in_event_loop(win),
                    };
                });
            }
        });

        preparing_internal.on_login({
            let win = win_weak.clone();
            let cfg = self.cfg.clone();
            let http_client = self.http_client.clone();

            move |username, password| {
                let win = win.clone();
                let cfg = cfg.clone();
                let http_client = http_client.clone();

                tokio::spawn(async move {
                    let base_path = cfg.read().await.server_api_config().base_path().to_owned();
                    match preparing::login(
                        &api_cfg(http_client, base_path),
                        api::models::UserLoginRequest::new(username.to_string(), password.to_string())
                    ).await {
                        Ok(jwt) => {
                            cfg.write().await.server_api_config_mut().set_jwt(jwt.as_str());
                            PreparingActions::InvokeNextState.run_in_event_loop(win);
                        },
                        Err(err) => MainActions::from(err).run_in_event_loop(win),
                    };
                });
            }
        });

        preparing_internal.on_register({
            let win = win_weak.clone();
            let cfg = self.cfg.clone();
            let http_client = self.http_client.clone();

            move |username, password, verify, key| {
                let win = win.clone();
                let cfg = cfg.clone();
                let http_client = http_client.clone();

                tokio::spawn(async move {
                    if password != verify {
                        MainActions::ShowNotification("Password and verify password not ident".to_owned(), String::default(), NotificationType::Info).run_in_event_loop(win);
                        return
                    }

                    let base_path = cfg.read().await.server_api_config().base_path().to_owned();
                    match preparing::register(
                        &api_cfg(http_client, base_path),
                        api::models::UserRegisterRequest::new(username.to_string(), password.to_string(), key.to_string())
                    ).await {
                        Ok(_) => PreparingActions::InvokeNextState.run_in_event_loop(win),
                        Err(err) => MainActions::from(err).run_in_event_loop(win),
                    };
                });
            }
        });
    }
}
