use {
    crate::{
        NotificationType, 
        actions::UiActions, 
        app::{Application, ApplicationConfig},
        service::auth
    },
    slint::ComponentHandle, 
    std::sync::Arc,
    tokio::sync::RwLock,
};

impl Application {
    pub fn init_auth_callbacks(&self, preparing_cfg: Arc<RwLock<ApplicationConfig>>, auth_service: Arc<auth::Authenticator>) {
        let win_weak = self.ui_window.as_weak();

        self.ui_window.on_login({
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

        self.ui_window.on_register({
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
    }
}
