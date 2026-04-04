use {
    crate::{
        NotificationType, 
        actions::UiActions, 
        app::Application,
        service::auth::Authenticator
    },
    slint::ComponentHandle, 
    std::sync::Arc,
    tokio::sync::RwLock,
};

impl Application {
    pub fn init_auth_callbacks(&self, auth_service: Arc<RwLock<Authenticator>>) {
        let win_weak = self.ui_window.as_weak();

        self.ui_window.on_login({
            let win = win_weak.clone();
            let service = auth_service.clone();
            let cfg = self.cfg.clone();

            move |username, password| {
                let win = win.clone();
                let service = service.clone();
                let cfg = cfg.clone();

                tokio::spawn(async move {
                    let (jwt, act) = service.read().await.login(
                        openapi::models::UserLoginRequest::new(username.to_string(), password.to_string())
                    ).await;

                    if let Some(jwt) = jwt {
                        cfg.write().await.server_api_config_mut().set_jwt(jwt.as_str());
                    }
                    act.run_in_event_loop(win);
                });
            }
        });

        self.ui_window.on_register({
            let win = win_weak.clone();
            let service = auth_service.clone();

            move |username, password, verify, key| {
                let win = win.clone();
                let service = service.clone();

                tokio::spawn(async move {
                    if password != verify {
                        UiActions::ShowNotification("Password and verify password not ident!".to_owned(), NotificationType::Error).run_in_event_loop(win);
                        return
                    }

                    service.read().await.register(openapi::models::UserRegisterRequest::new(username.to_string(), password.to_string(), key.to_string())).await.run_in_event_loop(win);
                });
            }
        });
    }
}
