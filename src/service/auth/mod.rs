pub mod extend;

use {
    super::ServiceError,
    crate::api::{self, auth_service_client::AuthServiceClient},
    tonic::transport::Channel,
};

pub struct Authenticator {
    client: AuthServiceClient<Channel>,
}

impl Authenticator {
    pub fn new(client: AuthServiceClient<Channel>) -> Self {
        Self { client }
    }

    pub async fn login(&mut self, user: api::User) -> Result<String, ServiceError> {
        match self.client.login(user).await {
            Ok(resp) => Ok(resp.get_ref().token.clone()),
            Err(status) => Err(ServiceError::from(status)),
        }
    }

    pub async fn register(&mut self, reg_req: api::RegisterRequest) -> Result<(), ServiceError> {
        match self.client.register(reg_req).await {
            Ok(_) => Ok(()),
            Err(status) => Err(ServiceError::from(status)),
        }
    }
}
