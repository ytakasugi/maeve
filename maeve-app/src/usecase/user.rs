use std::sync::Arc;

use derive_new::new;

use maeve_adapter::modules::RepositoriesModuleExt;
use maeve_kernel::repository::user::UserRepository;

use crate::model::user::CreateUser;


#[derive(new)]
pub struct UserUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>
}

impl<R: RepositoriesModuleExt> UserUseCase<R> {
    pub async fn create_user(&self, payload: CreateUser) -> anyhow::Result<()> {
        self.repositories
            .user_repository()
            .create(payload.try_into()?)
            .await
    }
}