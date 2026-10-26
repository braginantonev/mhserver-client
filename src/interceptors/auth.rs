use std::str::FromStr;
use tonic::metadata::AsciiMetadataValue;

pub struct AuthInterceptor {
    token: AsciiMetadataValue
}

impl AuthInterceptor {
    pub fn new(token: &str) -> Self {
        Self { token: AsciiMetadataValue::from_str(token).unwrap_or(AsciiMetadataValue::from_static("")), }
    }
}

impl tonic::service::Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
        let _ = request.metadata_mut().insert("authorization", self.token.clone());
        Ok(request)
    }
}   