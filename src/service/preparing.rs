use {
    super::ServiceError,
    crate::UpdateStatus, 
    api::{
        apis::{configuration::Configuration, default_api::{ping as tools_ping, users_login, users_register, version}}, 
        models::{UserLoginRequest, UserRegisterRequest},
    }, 
    semver::Version,
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

    let last_app_ver = match api_cfg.client.get("https://raw.githubusercontent.com/braginantonev/mhserver-client/main/VERSION").send().await {
        Ok(resp) => resp.text().await.unwrap(),
        Err(_) => return Ok(UpdateStatus::CantCheck)
    };

    if Version::parse(&last_app_ver).unwrap() > Version::parse(crate::APPLICATION_VERSION).unwrap() {
        return Ok(UpdateStatus::Available);
    }

    Ok(UpdateStatus::NotNeeded)
}

#[cfg(target_os = "windows")]
pub async fn download_update(api_cfg: &Configuration) -> Result<(), ServiceError> {
    use {
        futures_util::stream::StreamExt, system_interface::io::IoExt
    };

    let updated_app = match std::fs::File::create(std::env::temp_dir().join("mhserver-client.update")) {
        Ok(v) => v,
        Err(err) => return Err(ServiceError::new("failed create update file", Some(err.to_string()), None)),
    };

    let last_app_ver = match api_cfg.client.get("https://raw.githubusercontent.com/braginantonev/mhserver-client/main/VERSION").send().await {
        Ok(resp) => resp.text().await.unwrap(),
        Err(err) => return Err(ServiceError::new("failed download update", Some(err.to_string()), None)),
    };

    let mut stream = match api_cfg.client
        .get(format!("https://github.com/braginantonev/mhserver-client/releases/download/{last_app_ver}/mhserver-client.exe"))
        .header(reqwest::header::USER_AGENT, format!("mhserver-client-{}", crate::APPLICATION_VERSION))
        .send()
        .await
    {
        Ok(res) => res.bytes_stream(),
        Err(err) => return Err(ServiceError::new("failed download update", Some(err.to_string()), None)),
    };

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.unwrap(); //tmp
        let _ = updated_app.write_all(&chunk);
    }

    let _ = updated_app.flush();

    Ok(())
} 