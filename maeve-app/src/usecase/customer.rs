use std::sync::Arc;

use derive_new::new;

use maeve_adapter::modules::RepositoriesModuleExt;
use maeve_kernel::repository::customer::CustomerRepository;

use crate::model::customer::CreateCustomer;
use crate::model::customer::CustomerView;

#[derive(new)]
pub struct CustomerUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> CustomerUseCase<R> {
    pub async fn find_customer(&self, id: String) -> anyhow::Result<Option<CustomerView>> {
        let customer = self
            .repositories
            .customer_repository()
            .find(&id.try_into()?)
            .await?;

        match customer {
            Some(customer) => Ok(Some(CustomerView::new(customer))),
            None => Ok(None),
        }
    }

    pub async fn create_customer(&self, payload: CreateCustomer) -> anyhow::Result<()> {
        self.repositories
            .customer_repository()
            .create(payload.try_into()?)
            .await
    }
}
