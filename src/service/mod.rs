/// This crate used for service-requests to server.
/// All structs have initialized once config

use {
    crate::config::app::ApplicationConfig
};

pub mod auth;
pub mod files;

pub trait Service {
    fn update_config_from_app(&mut self, app_cfg: ApplicationConfig);
}