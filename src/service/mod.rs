/// This crate used for service-requests to server.
/// All structs have initialized once config

pub mod files;
pub mod preparing;

use api::apis::Error;

pub struct ServiceError {
    label: String,
    desc: Option<String>,
    code: Option<reqwest::StatusCode>,
}

impl ServiceError {
    pub fn new(label: &str, desc: Option<String>, code: Option<reqwest::StatusCode>) -> Self {
        Self { label: label.to_owned(), desc, code }
    }

    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn description(&self) -> String {
        match &self.desc {
            Some(v) => v.clone(),
            None => String::new()
        }
    }

    pub fn code(&self) -> Option<reqwest::StatusCode> {
        self.code
    }

    pub fn with_label(mut self, label: &str) -> Self {
        if !self.label.is_empty() {
            let old_label = &self.label.clone();
            self = self.with_desc(old_label);
        }
        self.label = label.to_owned();
        self
    }

    pub fn with_desc(mut self, desc: &str) -> Self {
        self.desc = Some(match self.desc {
            Some(mut v) => {
                v.push('\n');
                v.push_str(desc);
                v
            },
            None => desc.to_owned()
        });
        self
    }
}

impl<T> From<Error<T>> for ServiceError {
    fn from(value: Error<T>) -> Self {
        match value {
            Error::Reqwest(err) => ServiceError { label: String::from("network error"), desc: Some(err.to_string()), code: err.status() },
            Error::Serde(err) => ServiceError { label: String::from("request parse error"), desc: Some(err.to_string()), code: None },
            Error::Io(err) => ServiceError { label: err.to_string(), desc: None, code: None },
            Error::ResponseError(c) => {
                let label = c.content;
                ServiceError { label: String::from(label.strip_suffix('\n').unwrap_or(&label)), desc: None, code: Some(c.status) }
            },
        }
    }
}

pub trait Service {
    fn update_config(&mut self, http_client: reqwest::Client, app_cfg: crate::config::app::ApplicationConfig);
}