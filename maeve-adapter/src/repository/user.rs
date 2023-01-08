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

    async fn delete(&self, id: &Id<User>) -> anyhow::Result<()> {
        // コネクションプールをクローン
        let pool = self.pool.0.clone();
        // トランザクションを開始する
        let mut transaction = pool.begin().await.unwrap();

        // ユーザーを削除する
        sqlx::query_file_as!(
            UserTable,
            "sql/deleteUser.sql",
            id.value.to_string()
        )
        .execute(&mut transaction)
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

#[cfg(test)]
mod test {
    use maeve_kernel::model::user::NewUser;
    use maeve_kernel::model::Id;
    use maeve_kernel::repository::user::UserRepository;
    use ulid::Ulid;

    use crate::persistence::database::Db;
    
    use super::DatabaseRepository;

    #[tokio::test]
    async fn test_create_user() {
        let db = Db::new().await;
        let repository = DatabaseRepository::new(db);
        let id = Ulid::new();
        
        repository
            .create(NewUser::new(
                Id::new(id),
                "TestUser".to_string(),
                "TestPassword".to_string(),
                "Test".to_string()
            ))
            .await
            .unwrap();

        let find_user = repository.find(&Id::new(id)).await.unwrap().unwrap();

        assert_eq!(find_user.id.value, id);
    }

    #[tokio::test]
    async fn test_delete_user() {
        let db = Db::new().await;
        let repository = DatabaseRepository::new(db);
        let id = Ulid::new();

        repository
            .create(NewUser::new(
                Id::new(id),
                "TestUser".to_string(),
                "TestPassword".to_string(),
                "Test".to_string()
            ))
            .await
            .unwrap();

        repository
            .delete(&Id::new(id))
            .await
            .unwrap();

        let find_user = repository.find(&Id::new(id)).await.unwrap();

        assert_eq!(find_user, None);
    }
}