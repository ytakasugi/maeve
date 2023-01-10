use std::sync::Arc;

use derive_new::new;

use maeve_adapter::modules::RepositoriesModuleExt;
use maeve_kernel::{repository::customer::CustomerRepository};
use maeve_kernel::model::customer::NewCustomer;

#[derive(new)]
pub struct CustomerUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>
}

impl<R: RepositoriesModuleExt> CustomerUseCase<R> {
    pub async fn create_customer(&self, id: String, payload: NewCustomer) -> anyhow::Result<()> {
        self.repositories
            .customer_repository()
            .create(&id.try_into()?, payload)
            .await
    }
}