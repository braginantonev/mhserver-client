#![windows_subsystem = "windows"]

mod actions;
mod app;
mod config;
mod override_slint;
mod repository;
mod service;

slint::include_modules!();

pub mod api {
    tonic::include_proto!("auth");
    tonic::include_proto!("data");
}

pub const APPLICATION_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<(), app::errors::ApplicationError> {
    println!("mhserver client {APPLICATION_VERSION}");
    //println!("target api version: {}", api::API_VERSION);

    slint::BackendSelector::new()
        .backend_name("winit".to_owned())
        .require_opengl()
        .select()
        .expect("Unable to create Slint backend with OpenGL ES renderer");

    let app = app::Application::new()?;

    app.run().await?;

    Ok(())
}
