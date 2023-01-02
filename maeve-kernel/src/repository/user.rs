use async_trait::async_trait;

use crate::model::user::{User, CreateUser};

#[async_trait]
pub trait UserRepository {
    async fn find(&self, id: i32) -> anyhow::Result<User>;
    async fn create(&self, payload: CreateUser) -> anyhow::Result<CreateUser>;
}