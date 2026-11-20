use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub enum ErrorKind {
    Window,
}

#[derive(Debug)]
pub struct ApplicationError {
    kind: ErrorKind,
    description: String,
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "app error ({:?}): {}", self.kind, self.description)
    }
}

impl Error for ApplicationError {}

impl From<slint::PlatformError> for ApplicationError {
    fn from(value: slint::PlatformError) -> Self {
        Self { kind: ErrorKind::Window, description: value.to_string() }
    }
}
