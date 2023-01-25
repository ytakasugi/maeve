use std::sync::Arc;

use derive_new::new;

use maeve_adapter::modules::RepositoriesModuleExt;
use maeve_kernel::repository::user_detail::UserDetailRepository;

use crate::model::user_detail::UserDetailView;

#[derive(new)]
pub struct UserDetailUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> UserDetailUseCase<R> {
    pub async fn find_user_detail(
        &self,
        user_id: String,
    ) -> anyhow::Result<Option<Vec<UserDetailView>>> {
        let res = self
            .repositories
            .user_detail_repository()
            .find(&user_id.try_into()?)
            .await?;

        match res {
            Some(detail) => {
                let details = detail.into_iter().map(|view| view.into()).collect();
                Ok(Some(details))
            }
            None => Ok(None),
        }
    }
}
