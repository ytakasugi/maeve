use maeve_kernel::model::customer::Customer;
use maeve_kernel::model::user::User;
use maeve_kernel::model::user_detail::UserDetail;
use maeve_kernel::repository::{
    customer::CustomerRepository, user::UserRepository, user_detail::UserDetailRepository,
};

use crate::{persistence::database::Db, repository::DatabaseRepository};

pub struct RepositoriesModule {
    user_repository: DatabaseRepository<User>,
    customer_repository: DatabaseRepository<Customer>,
    user_detail_repository: DatabaseRepository<UserDetail>,
}

pub trait RepositoriesModuleExt {
    type UserRepo: UserRepository;
    type CustomerRepo: CustomerRepository;
    type UserDetailRepo: UserDetailRepository;

    fn user_repository(&self) -> &Self::UserRepo;
    fn customer_repository(&self) -> &Self::CustomerRepo;
    fn user_detail_repository(&self) -> &Self::UserDetailRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type UserRepo = DatabaseRepository<User>;
    type CustomerRepo = DatabaseRepository<Customer>;
    type UserDetailRepo = DatabaseRepository<UserDetail>;

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }

    fn customer_repository(&self) -> &Self::CustomerRepo {
        &self.customer_repository
    }

    fn user_detail_repository(&self) -> &Self::UserDetailRepo {
        &self.user_detail_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let user_repository = DatabaseRepository::new(db.clone());
        let customer_repository = DatabaseRepository::new(db.clone());
        let user_detail_repository = DatabaseRepository::new(db.clone());
        Self {
            user_repository,
            customer_repository,
            user_detail_repository,
        }
    }
}
