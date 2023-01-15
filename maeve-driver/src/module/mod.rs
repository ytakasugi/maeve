use std::sync::Arc;

use maeve_adapter::{
    modules::{RepositoriesModule, RepositoriesModuleExt},
    persistence::database::Db,
};

use maeve_app::usecase::{customer::CustomerUseCase, user::UserUseCase};

pub struct Modules {
    user_usecase: UserUseCase<RepositoriesModule>,
    customer_usecase: CustomerUseCase<RepositoriesModule>,
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn user_usecase(&self) -> &UserUseCase<Self::RepositoriesModule>;
    fn customer_usecase(&self) -> &CustomerUseCase<Self::RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn user_usecase(&self) -> &UserUseCase<Self::RepositoriesModule> {
        &self.user_usecase
    }

    fn customer_usecase(&self) -> &CustomerUseCase<Self::RepositoriesModule> {
        &self.customer_usecase
    }
}

impl Modules {
    pub async fn new() -> Modules {
        let db = Db::new().await;
        let repositories_module = Arc::new(RepositoriesModule::new(db));

        let user_usecase = UserUseCase::new(repositories_module.clone());
        let customer_usecase = CustomerUseCase::new(repositories_module.clone());

        Self {
            user_usecase,
            customer_usecase,
        }
    }
}
