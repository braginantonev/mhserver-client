/// This crate used for service-requests to server.
/// All structs have initialized once config
pub mod auth;
pub mod files;
pub mod preparing;

use tonic::Status;

#[derive(Clone)]
pub struct ServiceError {
    label: String,
    desc: Option<String>,
    code: Option<tonic::Code>,
}

impl ServiceError {
    pub fn new(label: &str, desc: Option<String>, code: Option<tonic::Code>) -> Self {
        Self {
            label: label.to_owned(),
            desc,
            code,
        }
    }

    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn description(&self) -> String {
        match &self.desc {
            Some(v) => v.clone(),
            None => String::new(),
        }
    }

    pub fn code(&self) -> Option<tonic::Code> {
        self.code
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.label = label.to_owned();
        self
    }

    pub fn with_desc(mut self, desc: &str) -> Self {
        self.desc = Some(match self.desc {
            Some(mut v) => {
                v.push('\n');
                v.push_str(desc);
                v
            }
            None => desc.to_owned(),
        });
        self
    }
}

impl From<Status> for ServiceError {
    fn from(value: Status) -> Self {
        Self::new(
            "service error",
            Some(value.message().to_owned()),
            Some(value.code()),
        )
    }
}
