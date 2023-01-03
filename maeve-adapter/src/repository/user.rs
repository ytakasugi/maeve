use async_trait::async_trait;
use maeve_kernel::{
    model::{
        user::{
            User,
            NewUser,
        },
        Id,
    },
    repository::user::UserRepository,
};

use crate::model::user::UserTable;
use super::DatabaseRepository;

#[async_trait]
impl UserRepository for DatabaseRepository<User> {
    async fn find(&self, id: &Id<User>) -> anyhow::Result<Option<User>> {
        let pool = self.pool.0.clone();

        let user_table = sqlx::query_file_as!(
                UserTable,
                "sql/findUser.sql",
                id.value.to_string()
            )
            .fetch_one(&*pool)
            .await
            .ok();

        match user_table {
            Some(user) => Ok(Some(user.try_into()?)),
            None => Ok(None)
        }
    }

    async fn create(&self, payload: NewUser) -> anyhow::Result<()> {
        let user_table: UserTable = payload.try_into()?;
        // コネクションプールをクローン
        let pool = self.pool.0.clone();
        // トランザクションを開始する
        let mut transaction = pool.begin().await.unwrap();

        // ユーザーを新規作成する
        let _ = sqlx::query_file_as!(
            UserTable,
            "sql/createUser.sql",
            user_table.id,
            user_table.user_name,
            user_table.password_hash,
            user_table.user_role
        )
        .fetch_one(&mut transaction)
        .await?;

        // トランザクションをコミットする
        transaction
            .commit()
            .await
            .unwrap_or_else(|_| {
                panic!("Commit failed.")
            });

        Ok(())
    }
}