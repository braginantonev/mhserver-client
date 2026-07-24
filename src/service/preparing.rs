use {
    crate::{
        NotificationType, actions::UiActions,
    },
    api::{
        apis::configuration::Configuration,
        apis::default_api::{users_login, users_register, ping as tools_ping},
        models::{UserLoginRequest, UserRegisterRequest},
    },
};

pub async fn login(api_cfg: &Configuration, user: UserLoginRequest) -> (Option<String>, UiActions) {
    match users_login(api_cfg, user).await {
        Ok(resp) => (Some(resp.content.unwrap()), todo!()/*UiActions::ChangePreparingState(PreparingStates::Login.next())*/),
        Err(err) => (None, UiActions::ShowNotification(err.to_string(), NotificationType::Error))
    }
}

pub async fn register(api_cfg: &Configuration, user: UserRegisterRequest) -> UiActions {
    match users_register(api_cfg, user).await {
        Ok(_) => todo!()/*UiActions::ChangePreparingState(PreparingStates::Login)*/,
        Err(err) => UiActions::ShowNotification(err.to_string(), NotificationType::Error)
    }
}

/// Ping server. Return true, if server available, and false, if not available
/// Use to ping server addr from self, if target is None
pub async fn ping(api_cfg: &Configuration, target: Option<&str>) -> bool {
    let mut local_api_conf = api_cfg.clone();

    if let Some(addr) = target {
        local_api_conf.base_path = addr.to_owned()
    }

    match tools_ping(&local_api_conf).await {
        Ok(_) => true,
        Err(err) => {
            println!("Error: {}", err.to_string());
            false
        }
    }
}