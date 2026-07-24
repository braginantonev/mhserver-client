mod config;
mod app;
mod service;
mod notification;
mod actions;
mod repository;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), app::errors::ApplicationError> {
    let mut app = app::Application::new()?;
    
    app.run().await?;

    Ok(())
}