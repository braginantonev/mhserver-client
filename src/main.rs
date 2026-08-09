#![windows_subsystem = "windows"]

mod config;
mod app;
mod service;
mod actions;
mod repository;

slint::include_modules!();

pub const APPLICATION_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<(), app::errors::ApplicationError> {
    println!("mhserver client {APPLICATION_VERSION}");
    println!("target api version: {}", api::API_VERSION);

    slint::BackendSelector::new()
        .backend_name("winit".to_owned())
        .require_opengl()
        .select()
        .expect("Unable to create Slint backend with OpenGL ES renderer");
    
    let app = app::Application::new()?;
    
    app.run().await?;

    Ok(())
}