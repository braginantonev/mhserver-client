pub mod ping;
mod endpoints;

struct ServerError(String);

impl ServerError {
    fn new(description: &str) -> Self {
        ServerError(description.to_string())
    }
}