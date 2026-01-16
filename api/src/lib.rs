pub mod ping;
mod endpoints;

pub struct ServerError(String);

impl ServerError {
    pub fn new(description: &str) -> Self {
        ServerError(description.to_string())
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    } 
}