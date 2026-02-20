mod config;
mod app;
mod service;
mod notification;
mod actions;

slint::include_modules!();

impl PreparingStates {
    pub fn next(&self) -> PreparingStates {
        match self {
            // Checks
            PreparingStates::CheckConn => PreparingStates::CheckAuth,
            PreparingStates::CheckAuth => PreparingStates::End,

            // Change states
            PreparingStates::Normal => PreparingStates::CheckConn, // First check
            PreparingStates::Connection => PreparingStates::CheckAuth,
            PreparingStates::Login => PreparingStates::End, // Last check

            _ => todo!(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), app::errors::ApplicationError> {
    let mut app = app::Application::new()?;
    
    app.run().await?;

    Ok(())
}