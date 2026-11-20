#![windows_subsystem = "windows"]

mod actions;
mod app;
mod config;
mod override_slint;
mod repository;
mod service;
mod interceptors;

slint::include_modules!();

pub mod api {
    pub mod v3 {
        tonic::include_proto!("auth.v3");
        tonic::include_proto!("files.v3");
    }
}

pub const APPLICATION_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<(), app::errors::ApplicationError> {
    println!("mhserver client {APPLICATION_VERSION}");

    rustls::crypto::ring::default_provider().install_default().expect("Failed to install ring crypto provider");

    slint::BackendSelector::new()
        .backend_name("winit".to_owned())
        .require_opengl()
        .select()
        .expect("Unable to create Slint backend with OpenGL ES renderer");

    let app = app::Application::new()?;

    app.run().await?;

    Ok(())
}
