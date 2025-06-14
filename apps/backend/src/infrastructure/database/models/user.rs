use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use uuid::Uuid; // Add this line

use crate::domain::entities::user::User;

use super::identity_link;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)] // Set auto_increment to false
    pub id: Uuid, // Changed from i32
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    IdentityLink,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::IdentityLink => Entity::has_many(identity_link::Entity).into(),
        }
    }
}

impl Related<identity_link::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IdentityLink.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for User {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            created_at: model.created_at,
            updated_at: model.updated_at,
            identity_links: vec![],
        }
    }
}
