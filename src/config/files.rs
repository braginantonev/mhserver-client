use api::apis::configuration::Configuration;

#[derive(Default, Debug)]
pub struct FileServiceConfig {
    pub api_conf: Configuration,
}

impl FileServiceConfig {
    pub fn new(api_conf: Configuration) -> Self {
        Self { api_conf: api_conf }
    }
}