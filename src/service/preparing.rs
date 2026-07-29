use {
    super::ServiceError,
    api::{
        apis::configuration::Configuration,
        apis::default_api::{users_login, users_register, ping as tools_ping, version},
        models::{UserLoginRequest, UserRegisterRequest},
    },
    semver::Version
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

pub enum UpdateStatus {
    Required,
    Available,
    ServerOld,
    CantCheck,
    NotNeeded
}

pub async fn update_status(api_cfg: &Configuration) -> Result<UpdateStatus, ServiceError> {
    let server_version = match version(api_cfg).await {
        Ok(v) => v.content.unwrap(),
        Err(err) => return Err(ServiceError::from(err)),
    };

    let server_version = match Version::parse(&server_version) {
        Ok(v) => v,
        Err(err) => return Err(ServiceError::new("failed check update", Some(err.to_string()), None)),
    };

    let current_api = Version::parse(api::API_VERSION).unwrap();
    
    if server_version.major < current_api.major {
        return Ok(UpdateStatus::ServerOld);
    }

    if server_version.major > current_api.major {
        return Ok(UpdateStatus::Required)
    }

    if server_version.minor > current_api.minor {
        return Ok(UpdateStatus::Available)
    }

    let last_app_ver = match reqwest::get("https://github.com/braginantonev/mhserver-client/blob/main/VERSION").await {
        Ok(resp) => resp.text().await.unwrap(),
        Err(_) => return Ok(UpdateStatus::CantCheck)
    };

    if Version::parse(&last_app_ver).unwrap() > Version::parse(crate::APPLICATION_VERSION).unwrap() {
        return Ok(UpdateStatus::Available);
    }

    Ok(UpdateStatus::NotNeeded)
}