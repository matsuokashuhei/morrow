use anyhow::Result;
use std::sync::Arc;

use crate::{
    application::dtos::authentication_dto::SignOutInputDTO,
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

    pub async fn execute(&self, input: SignOutInputDTO) -> Result<()> {
        self.authentication_service
            .sign_out(&input.username)
            .await?;
        Ok(())
    }
}
