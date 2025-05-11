use anyhow::Result;
use std::sync::Arc;

use crate::{
    application::dtos::authentication_dto::SignOutDTO,
    domain::services::authentication_service::AuthenticationService,
};

pub struct SignOut {
    authentication_service: Arc<dyn AuthenticationService>,
}

impl SignOut {
    pub fn new(authentication_service: Arc<dyn AuthenticationService>) -> Self {
        Self {
            authentication_service,
        }
    }

    pub async fn execute(&self, input: SignOutDTO) -> Result<()> {
        self.authentication_service
            .sign_out(&input.access_token)
            .await?;
        Ok(())
    }
}
