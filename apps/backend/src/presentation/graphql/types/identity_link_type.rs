use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::application::dtos::identity_link_dto::IdentityLinkDto;

#[derive(SimpleObject)]
pub struct IdentityLink {
    pub id: Uuid,
    pub provider: String,
    pub sub: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<IdentityLinkDto> for IdentityLink {
    fn from(dto: IdentityLinkDto) -> Self {
        Self {
            id: dto.id,
            provider: dto.provider,
            sub: dto.sub,
            user_id: dto.user_id,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}
