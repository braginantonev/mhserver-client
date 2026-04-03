
use openapi::{
    apis::configuration::Configuration,
    apis::default_api,
};

pub struct ServerTools {
    api_conf: Configuration,
}

impl ServerTools {
    pub fn new(api_conf: Configuration) -> ServerTools {
        Self { api_conf }
    }

    /// Ping server. Return true, if server available, and false, if not available
    /// Use to ping server addr from self, if target is None
    pub async fn ping(&self, target: Option<&str>) -> bool {
        let mut local_api_conf = self.api_conf.clone();

        if let Some(addr) = target {
            local_api_conf.base_path = addr.to_owned()
        }

        println!("{:?}", local_api_conf);

        match default_api::ping(&local_api_conf).await {
            Ok(_) => true,
            Err(err) => {
                println!("Error: {}", err.to_string());
                false
            }
        }
    }
}

impl crate::service::Service for ServerTools {
    fn update_config_from_app(&mut self, app_cfg: crate::config::app::ApplicationConfig) {
        let server_api_conf = app_cfg.server_api_config();

        self.api_conf.base_path = server_api_conf.base_path().to_owned();
        self.api_conf.bearer_access_token = Some(server_api_conf.jwt().to_owned());
    }
}