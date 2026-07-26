mod config;
mod app;
mod service;
mod actions;
mod repository;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), app::errors::ApplicationError> {
    let app = app::Application::new()?;
    
    app.run().await?;

    Ok(())
}