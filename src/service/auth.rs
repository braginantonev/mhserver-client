use {
    crate::{
        NotificationType, PreparingStates, actions::UiActions,
    },
    api::{
        apis::configuration::Configuration,
        apis::default_api::{users_login, users_register},
        models::{UserLoginRequest, UserRegisterRequest},
    },
};

pub struct Authenticator {
    api_conf: Configuration,
}

impl Authenticator {
    pub fn new(api_conf: Configuration) -> Self {
        Self { api_conf }
    }

    pub async fn login(&self, user: UserLoginRequest) -> (Option<String>, UiActions) {
        match users_login(&self.api_conf, user).await {
            Ok(resp) => (Some(resp.content.unwrap()), UiActions::ChangePreparingState(PreparingStates::Login.next())),
            Err(err) => (None, UiActions::ShowNotification(err.to_string(), NotificationType::Error))
        }
    }

    pub async fn register(&self, user: UserRegisterRequest) -> UiActions {
        match users_register(&self.api_conf, user).await {
            Ok(_) => UiActions::ChangePreparingState(PreparingStates::Login),
            Err(err) => UiActions::ShowNotification(err.to_string(), NotificationType::Error)
        }
    }
}

impl super::Service for Authenticator {
    fn update_config_from_app(&mut self, app_cfg: crate::config::app::ApplicationConfig) {
        self.api_conf.base_path = app_cfg.server_api_config().base_path().to_owned();
    }
}