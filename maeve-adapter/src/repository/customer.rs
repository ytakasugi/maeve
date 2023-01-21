use async_trait::async_trait;
use maeve_kernel::{
    model::{
        customer::{Customer, NewCustomer},
        Id,
    },
    repository::customer::CustomerRepository,
};

use super::DatabaseRepository;
use crate::model::customer::CustomerTable;

#[async_trait]
impl CustomerRepository for DatabaseRepository<Customer> {
    async fn find(&self, id: &Id<Customer>) -> anyhow::Result<Option<Customer>> {
        let pool = self.pool.0.clone();

        let customer_table =
            sqlx::query_file_as!(CustomerTable, "sql/findCustomer.sql", id.value.to_string())
                .fetch_one(&*pool)
                .await
                .ok();

        match customer_table {
            Some(customer) => Ok(Some(customer.try_into()?)),
            None => Ok(None),
        }
    }

    async fn create(&self, payload: NewCustomer) -> anyhow::Result<()> {
        let customer_table: CustomerTable = payload.try_into()?;
        let pool = self.pool.0.clone();
        let mut transaction = pool.begin().await.unwrap();

        let _ = sqlx::query_file_as!(
            CustomerTable,
            "sql/createCustomer.sql",
            customer_table.id,
            customer_table.user_id,
            customer_table.name,
            customer_table.zip_code,
            customer_table.address,
            customer_table.phone
        )
        .execute(&mut transaction)
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
        let user_id = Ulid::new();

        repository
            .create(NewCustomer::new(
                Id::new(id),
                Id::new(user_id),
                "TestCustomer".to_string(),
                "100-0014".to_string(),
                "TestCustomerAddress".to_string(),
                "999-9999-9999".to_string(),
            ))
            .await
            .unwrap();

        let find_customer = repository.find(&Id::new(id)).await.unwrap().unwrap();

        assert_eq!(find_customer.id.value, id);
    }
}
