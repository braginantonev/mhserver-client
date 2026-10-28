use {
    crate::UpdateStatus,
    super::ServiceError,
};

pub async fn check_update() -> Result<UpdateStatus, ServiceError> {
    use semver::Version;
    use std::cmp::Ordering; 
    
    let last_app_ver = match reqwest::get("https://raw.githubusercontent.com/braginantonev/mhserver-client/main/VERSION").await {
        Ok(resp) => resp.text().await.unwrap(),
        Err(err) => return Err(ServiceError::new("failed check update", Some(err.to_string()), None))
    };

    let last_app_ver = Version::parse(&last_app_ver).map_err(|err| -> ServiceError {
        ServiceError::new("bad server version", Some(err.to_string()), None)
    })?;

    Ok(match Version::parse(crate::APPLICATION_VERSION).unwrap().cmp(&last_app_ver) {
        Ordering::Less => UpdateStatus::Available,
        Ordering::Equal => UpdateStatus::NotNeeded,
        Ordering::Greater => UpdateStatus::NotNeeded,
    })
}

pub async fn download_update() -> Result<(), ServiceError> {
    if !cfg!(target_os = "windows") {
        return Err(ServiceError::new("failed download update", Some("This action available only in widnows".to_owned()), None)
            .with_desc("Please use your package manager or another way to update"))
    }

    use system_interface::io::IoExt; 

    let client = reqwest::Client::builder()
        .user_agent(format!("mhserver-client-{}", crate::APPLICATION_VERSION))
        .build()
        .map_err(|err| -> ServiceError {
            ServiceError::new("failed check update", Some(err.to_string()), None)
                .with_desc("failed create http client")
        })?;

    let last_app_ver = match client.get("https://raw.githubusercontent.com/braginantonev/mhserver-client/main/VERSION").send().await {
        Ok(resp) => resp.text().await.unwrap(),
        Err(err) => {
            return Err(ServiceError::new(
                "failed download update",
                Some(err.to_string()),
                None,
            ))
        }
    };

    let update_data = match reqwest::get(format!("https://github.com/braginantonev/mhserver-client/releases/download/{last_app_ver}/mhserver-client.exe")).await {
        Ok(res) => res.bytes().await.map_err(|err| -> ServiceError {
            ServiceError::new("failed download update", Some(err.to_string()), None)
        })?,
        Err(err) => return Err(ServiceError::new("failed download update", Some(err.to_string()), None)),
    };

    let update_file = std::fs::File::create(std::env::temp_dir().join("mhserver-client.update"))
        .map_err(|err| -> ServiceError {
            ServiceError { label: "failed download new version".to_owned(), desc: Some(err.to_string()), code: None }
        })?;

    update_file.write_all(&update_data).map_err(|err| -> ServiceError {
        ServiceError::new("failed save new version", Some(err.to_string()), None)
    })?;

    let _ = update_file.flush();

    Ok(())
}
