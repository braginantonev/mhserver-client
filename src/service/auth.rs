use {
    crate::{
        NotificationType, PreparingStates, actions::UiActions,
    },
    api::auth::*,
    reqwest::Client,
};

pub struct Authenticator {
    client: Client,
}

impl Authenticator {
    fn cloned_client(&self) -> Client {
        self.client.clone()
    }

    pub fn new(http_client: Client) -> Self {
        Self { client: http_client }
    }

    pub async fn login(&self, srv_addr: &str, user: User) -> (Option<String>, UiActions) {
        match login_v1(self.cloned_client(), srv_addr, user).await {
            Ok(jwt) => (Some(jwt), UiActions::ChangePreparingState(PreparingStates::Login.next())),
            Err(err) => (None, UiActions::ShowNotification(err.to_string(), NotificationType::Error))
        }
    }

    pub async fn register(&self, srv_addr: &str, user: User) -> UiActions {
        match register_v1(self.cloned_client(), srv_addr, user).await {
            Ok(_) => UiActions::ChangePreparingState(PreparingStates::Login),
            Err(err) => UiActions::ShowNotification(err.to_string(), NotificationType::Error)
        }
    }
}

impl super::Service for Authenticator {
    //Todo: delete this shit
    fn update_config_from_app(&mut self, _app_cfg: crate::config::app::ApplicationConfig) {}
}