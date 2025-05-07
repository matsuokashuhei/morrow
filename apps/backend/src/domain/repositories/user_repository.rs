use crate::domain::entities::user::{NewUser, User};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create(&self, user: NewUser) -> anyhow::Result<User>;
    async fn find_by_id(&self, id: i32) -> anyhow::Result<Option<User>>;
    async fn find_all(&self) -> anyhow::Result<Vec<User>>;
    async fn update(&self, user: User) -> anyhow::Result<User>;
    async fn delete(&self, id: i32) -> anyhow::Result<()>;
}
