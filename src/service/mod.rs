/// This crate used for service-requests to server.
/// All structs have initialized once config

pub mod files;
pub mod preparing;

pub trait Service {
    fn update_config(&mut self, app_cfg: crate::config::app::ApplicationConfig);
}