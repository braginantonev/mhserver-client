use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ApplicationErrors {
    FailedCreateWindow(String),
    FailedCreateHttpClient(String),
    WindowError(String)
}

impl ApplicationErrors {
    fn name<'a>(&self) -> &'a str {
        match self {
            ApplicationErrors::FailedCreateWindow(_) => "failed create main window",
            ApplicationErrors::FailedCreateHttpClient(_) => "failed create client window",
            ApplicationErrors::WindowError(_) => "window error",
        }
    }

    fn desc(&self) -> &str {
        match self {
            ApplicationErrors::FailedCreateWindow(desc) => desc,
            ApplicationErrors::FailedCreateHttpClient(desc) => desc,
            ApplicationErrors::WindowError(desc) => desc,
        }.as_str()
    }
}

#[derive(Debug)]
pub struct ApplicationError {
    err: ApplicationErrors
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(f, "app error: {}\n", self.err.name());
        write!(f, "error description: {}", self.err.desc())
    }
}

impl Error for ApplicationError {}

impl ApplicationError {
    pub fn new(err: ApplicationErrors) -> Self {
        Self { err: err }
    }
}