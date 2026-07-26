use {
    super::ServiceError,
    api::{
        apis::configuration::Configuration,
        apis::default_api::{users_login, users_register, ping as tools_ping},
        models::{UserLoginRequest, UserRegisterRequest},
    },
};

pub async fn login(api_cfg: &Configuration, user: UserLoginRequest) -> Result<String, ServiceError> {
    match users_login(api_cfg, user).await {
        Ok(resp) => Ok(resp.content.unwrap()),
        Err(err) => Err(ServiceError::from(err))
    }
}

pub async fn register(api_cfg: &Configuration, user: UserRegisterRequest) -> Result<(), ServiceError> {
    match users_register(api_cfg, user).await {
        Ok(_) => Ok(()),
        Err(err) => Err(ServiceError::from(err)),
    }
}

pub async fn ping(api_cfg: &Configuration) -> Result<(), ServiceError> {
    match tools_ping(api_cfg).await {
        Ok(_) => Ok(()),
        Err(err) => Err(ServiceError::from(err)),
    }
}