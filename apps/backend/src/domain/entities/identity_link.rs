use chrono::{DateTime, Utc};
use sea_orm::ActiveValue;
use uuid::Uuid;

use crate::infrastructure::database::models::identity_link;

#[derive(Debug, Clone)]
pub struct NewIdentityLink {
    pub provider: String,
    pub sub: String,
    pub user_id: Uuid, // Changed from i32
}

impl From<NewIdentityLink> for identity_link::ActiveModel {
    fn from(user: NewIdentityLink) -> Self {
        identity_link::ActiveModel {
            id: ActiveValue::NotSet,
            provider: ActiveValue::Set(user.provider),
            sub: ActiveValue::Set(user.sub),
            user_id: ActiveValue::Set(user.user_id),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct IdentityLink {
    pub id: Uuid, // Changed from i32
    pub provider: String,
    pub sub: String,
    pub user_id: Uuid, // Changed from i32
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
