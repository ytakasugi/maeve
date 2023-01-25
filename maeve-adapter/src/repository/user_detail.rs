use async_trait::async_trait;

use maeve_kernel::model::user_detail::UserDetail;
use maeve_kernel::model::Id;
use maeve_kernel::repository::user_detail::UserDetailRepository;

use super::DatabaseRepository;
use crate::model::user_detail::UserDetailTable;

#[async_trait]
impl UserDetailRepository for DatabaseRepository<UserDetail> {
    async fn find(&self, user_id: &Id<UserDetail>) -> anyhow::Result<Option<Vec<UserDetail>>> {
        let pool = self.pool.0.clone();

        let user_detail_table = sqlx::query_file_as!(
            UserDetailTable,
            "sql/findCustomerByUserId.sql",
            user_id.value.to_string()
        )
        .fetch_all(&*pool)
        .await
        .ok();

        match user_detail_table {
            Some(detail) => {
                let user_detail = detail.into_iter().flat_map(|d| d.try_into()).collect();
                Ok(Some(user_detail))
            }
            None => Ok(None),
        }
    }
}
