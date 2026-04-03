/// This crate used for service-requests to server.
/// All structs have initialized once config

pub mod auth;
pub mod files;
pub mod tools;

pub trait Service {
    fn update_config_from_app(&mut self, app_cfg: crate::config::app::ApplicationConfig);
}