use chrono::{DateTime, Utc};
use sea_orm::ActiveValue;

use crate::infrastructure::database::models::oauth_user;

#[derive(Debug, Clone)]
pub struct NewOAuthUser {
    pub provider: String,
    pub sub: String,
    pub user_id: i32,
}

impl From<NewOAuthUser> for oauth_user::ActiveModel {
    fn from(user: NewOAuthUser) -> Self {
        oauth_user::ActiveModel {
            id: ActiveValue::NotSet,
            provider: ActiveValue::Set(user.provider),
            sub: ActiveValue::Set(user.sub),
            user_id: ActiveValue::Set(user.user_id),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct OAuthUser {
    pub id: i32,
    pub provider: String,
    pub sub: String,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
