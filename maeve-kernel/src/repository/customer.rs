use async_trait::async_trait;

use crate::model::customer::Customer;
use crate::model::customer::NewCustomer;
use crate::model::Id;

#[async_trait]
pub trait CustomerRepository {
    async fn find(&self, id: &Id<Customer>) -> anyhow::Result<Option<Customer>>;
    //async fn find_by_user_id(&self, user_id: &Id<Customer>) -> anyhow::Result<Option<Customer>>
    async fn create(&self, payload: NewCustomer) -> anyhow::Result<()>;
    //async fn delete(&self, id: &Id<Customer>) -> anyhow::Result<()>;
}
