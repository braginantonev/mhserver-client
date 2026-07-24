/// This crate used for service-requests to server.
/// All structs have initialized once config

pub mod files;
pub mod preparing;

pub trait Service {
    fn update_config(&mut self, http_client: reqwest::Client, app_cfg: crate::config::app::ApplicationConfig);
}