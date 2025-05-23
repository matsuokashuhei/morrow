use crate::domain::entities::identity_link::{IdentityLink, NewIdentityLink};
use async_trait::async_trait;

#[async_trait]
pub trait IdentityLinkRepository: Send + Sync + 'static {
    async fn create(&self, user: NewIdentityLink) -> anyhow::Result<IdentityLink>;
    async fn find_by_sub(&self, sub: &str) -> anyhow::Result<IdentityLink>;
}
