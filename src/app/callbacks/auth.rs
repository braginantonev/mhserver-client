use slint::ComponentHandle;
use crate::{NotificationType, PreparingInternal, actions::{MainActions, PreparingActions, UiActions}, api::{self, auth_service_client::AuthServiceClient}, app::Application, service::auth::Authenticator};

impl Application {
    pub fn init_auth(&self) {
        let win_weak = self.ui_window.as_weak();
        let preparing_internal = self.ui_window.global::<PreparingInternal>();
        let chan = self.chan.clone();

        preparing_internal.on_login({
            let win = win_weak.clone();
            let cfg = self.cfg.clone();
            let chan = chan.clone();

            move |username, password| {
                let win = win.clone();
                let cfg = cfg.clone();

                let mut client = Authenticator::new(AuthServiceClient::new(chan.get().unwrap().clone()));

                tokio::spawn(async move {
                    match client.login(api::User {
                        name: username.into(),
                        password: password.into(),
                    }).await {
                        Ok(jwt) => {
                            cfg.write().await.server_api_config_mut().set_jwt(jwt.as_str());
                            PreparingActions::InvokeNextState.run_in_event_loop(win);
                        },
                        Err(err) => MainActions::from(err).run_in_event_loop(win),
                    }
                });
            }
        });

        preparing_internal.on_register({
            let win = win_weak.clone();
            let chan = chan.clone();

            move |username, password, verify, key| {
                let win = win.clone();

                if password != verify {
                    MainActions::ShowNotification("Password and verify password not ident".to_owned(), String::default(), NotificationType::Info).run_in_event_loop(win);
                    return
                }

                let mut client = Authenticator::new(AuthServiceClient::new(chan.get().unwrap().clone()));

                tokio::spawn(async move {
                    match client.register(api::RegisterRequest{
                        user: Some(api::User{
                            name: username.into(),
                            password: password.into(),
                        }),
                        secret_key: key.into()
                    }
                    ).await {
                        Ok(_) => PreparingActions::InvokeNextState.run_in_event_loop(win),
                        Err(err) => MainActions::from(err).run_in_event_loop(win),
                    };
                });
            }
        });
    }
}
