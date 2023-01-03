use maeve_kernel::repository::user::UserRepository;
use maeve_kernel::model::user::User;

use crate::{persistence::database::Db, repository::DatabaseRepository};

pub struct RepositoriesModule {
    user_repository: DatabaseRepository<User>,
}

pub trait RepositoriesModuleExt {
    type UserRepo: UserRepository;

    fn user_repository(&self) -> &Self::UserRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type UserRepo = DatabaseRepository<User>;

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let user_repository = DatabaseRepository::new(db);

        Self {
            user_repository,
        }
    }
}