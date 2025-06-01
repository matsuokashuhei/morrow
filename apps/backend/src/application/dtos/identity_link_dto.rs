use crate::domain::entities::identity_link::IdentityLink;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdentityLinkDto {
    pub id: Uuid, // Changed from i32
    pub provider: String,
    pub sub: String,
    pub user_id: Uuid, // Changed from i32
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<IdentityLink> for IdentityLinkDto {
    fn from(authorization_user: IdentityLink) -> Self {
        Self {
            id: authorization_user.id, // Correctly map id from IdentityLink
            provider: authorization_user.provider,
            sub: authorization_user.sub,
            user_id: authorization_user.user_id,
            created_at: authorization_user.created_at,
            updated_at: authorization_user.updated_at,
        }
    }
}
