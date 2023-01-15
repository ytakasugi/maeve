use async_trait::async_trait;

use crate::model::customer::NewCustomer;

#[async_trait]
pub trait CustomerRepository {
    //async fn find(&self, id: &Id<Customer>) -> anyhow::Result<Option<Customer>>;
    async fn create(&self, payload: NewCustomer) -> anyhow::Result<()>;
    //async fn delete(&self, id: &Id<Customer>) -> anyhow::Result<()>;
}
