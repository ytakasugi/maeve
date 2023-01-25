use async_trait::async_trait;

use crate::model::user_detail::UserDetail;
use crate::model::Id;

#[async_trait]
pub trait UserDetailRepository {
    async fn find(&self, user_id: &Id<UserDetail>) -> anyhow::Result<Option<Vec<UserDetail>>>;
}
