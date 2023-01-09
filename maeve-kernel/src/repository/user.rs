use async_trait::async_trait;

use crate::model::user::{User, NewUser};
use crate::model::Id;

#[async_trait]
pub trait UserRepository {
    async fn find(&self, id: &Id<User>) -> anyhow::Result<Option<User>>;
    async fn create(&self, payload: NewUser) -> anyhow::Result<()>;
    async fn delete(&self, id: &Id<User>) -> anyhow::Result<()>;
}