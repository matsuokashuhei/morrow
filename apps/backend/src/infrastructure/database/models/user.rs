use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

use crate::domain::entities::user::User;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for User {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
