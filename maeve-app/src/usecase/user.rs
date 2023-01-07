use std::sync::Arc;

use anyhow::Ok;
use derive_new::new;

use maeve_adapter::modules::RepositoriesModuleExt;
use maeve_kernel::repository::user::UserRepository;

use crate::model::user::UserView;
use crate::model::user::CreateUser;


#[derive(new)]
pub struct UserUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>
}

impl<R: RepositoriesModuleExt> UserUseCase<R> {
    pub async fn find_user(&self, id: String) -> anyhow::Result<Option<UserView>> {
        let user = self
            .repositories
            .user_repository()
            .find(&id.try_into()?)
            .await?;

        match user {
            Some(user) => {
                Ok(Some(UserView::new(user)))
            }
            None => Ok(None)
        }
    }

    pub async fn create_user(&self, payload: CreateUser) -> anyhow::Result<()> {
        self.repositories
            .user_repository()
            .create(payload.try_into()?)
            .await
    }

    pub async fn delete_user(&self, id: String) -> anyhow::Result<()> {
        self.repositories
            .user_repository()
            .delete(&id.try_into()?)
            .await
    }
}