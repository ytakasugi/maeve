use async_trait::async_trait;
use maeve_kernel::{
    model::{
        customer::{
            Customer,
            NewCustomer,
        },
        Id,
    },
    repository::customer::CustomerRepository,
};

use crate::model::customer::CustomerTable;
use super::DatabaseRepository;

#[async_trait]
impl  CustomerRepository for DatabaseRepository<Customer> {
    async fn create(&self, id: &Id<Customer>, payload: NewCustomer) -> anyhow::Result<()> {
        let pool = self.pool.0.clone();
        let mut transaction = pool.begin().await.unwrap();

        let _ = sqlx::query_file_as!(
            CustomerTable,
            "sql/createCustomer.sql",
            id.value.to_string(),
            payload.name,
            payload.email,
            payload.address,
            payload.phone
        )
        .fetch_one(&mut transaction)
        .await?;

        transaction
            .commit()
            .await
            .unwrap_or_else(|_| {
                panic!("Commit failed.")
            });
        
        Ok(())
    }
    
}