use async_trait::async_trait;
use maeve_kernel::{
    model::user::{
        User,
        CreateUser,
    },
    repository::user::UserRepository,
};

use super::DatabaseRepository;

#[async_trait]
impl UserRepository for DatabaseRepository<User> {
    async fn find(&self, id: i32) -> anyhow::Result<User> {
        let pool = self.pool.0.clone();

        let user = sqlx::query_file_as!(
                User,
                "sql/findUser.sql",
                id
            )
            .fetch_one(&*pool)
            .await
            .unwrap_or_else(|_| {
                panic!("Failed to find User.")
            });
        Ok(user)
    }

    async fn create(&self, payload: CreateUser) -> anyhow::Result<CreateUser> {
        // コネクションプールをクローン
        let pool = self.pool.0.clone();
        // トランザクションを開始する
        let mut transaction = pool.begin().await.unwrap();

        // ユーザーを新規作成する
        let user = sqlx::query_file_as!(
            CreateUser,
            "sql/createUser.sql",
            payload.user_name.clone(),
            payload.password_hash.clone(),
            payload.user_role.clone()
        )
        .fetch_one(&mut transaction)
        .await
        .unwrap_or_else(|_| {
            panic!("Failed to create User.")
        });

        // トランザクションをコミットする
        transaction
            .commit()
            .await
            .unwrap_or_else(|_| {
                panic!("Commit failed.")
            });

        Ok(user)
    }
}