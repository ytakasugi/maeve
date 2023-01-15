use async_trait::async_trait;
use maeve_kernel::{
    model::{
        customer::{Customer, NewCustomer},
        user::User,
        Id,
    },
    repository::customer::CustomerRepository,
};

use super::DatabaseRepository;
use crate::model::customer::CustomerTable;

#[async_trait]
impl CustomerRepository for DatabaseRepository<Customer> {
    async fn create(&self, id: &Id<User>, payload: NewCustomer) -> anyhow::Result<()> {
        let pool = self.pool.0.clone();
        let mut transaction = pool.begin().await.unwrap();

        let _ = sqlx::query_file_as!(
            CustomerTable,
            "sql/createCustomer.sql",
            id.value.to_string(),
            payload.name,
            payload.zip_code,
            payload.address,
            payload.phone
        )
        .fetch_one(&mut transaction)
        .await?;

        transaction
            .commit()
            .await
            .unwrap_or_else(|_| panic!("Commit failed."));

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use maeve_kernel::model::customer::NewCustomer;
    use maeve_kernel::model::Id;
    use maeve_kernel::repository::customer::CustomerRepository;

    use ulid::Ulid;

    use crate::persistence::database::Db;

    use super::DatabaseRepository;

    #[tokio::test]
    async fn test_create_customer() {
        let db = Db::new().await;
        let repository = DatabaseRepository::new(db);
        let id = Ulid::new();

        repository
            .create(
                &Id::new(id),
                NewCustomer::new(
                    "TestCustomer".to_string(),
                    "100-0014".to_string(),
                    "TestCustomerAddress".to_string(),
                    "999-9999-9999".to_string(),
                ),
            )
            .await
            .unwrap();
    }
}
