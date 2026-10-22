pub mod extend;

use {
    super::ServiceError,
    crate::api::{self, auth_service_client::AuthServiceClient},
    tonic::transport::Channel,
};

pub struct Authenticator;

impl Authenticator {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn login(&self, mut client: AuthServiceClient<Channel>, user: api::User) -> Result<String, ServiceError> {
        match client.login(user).await {
            Ok(resp) => Ok(resp.get_ref().token.clone()),
            Err(status) => Err(ServiceError::from(status)),
        }
    }

    pub async fn register(&self, mut client: AuthServiceClient<Channel>, reg_req: api::RegisterRequest) -> Result<(), ServiceError> {
        match client.register(reg_req).await {
            Ok(_) => Ok(()),
            Err(status) => Err(ServiceError::from(status)),
        }
    }
}
