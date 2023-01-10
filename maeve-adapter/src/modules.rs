use maeve_kernel::model::customer::Customer;
use maeve_kernel::repository::{user::UserRepository, customer::CustomerRepository};
use maeve_kernel::model::user::User;

use crate::{persistence::database::Db, repository::DatabaseRepository};

pub struct RepositoriesModule {
    user_repository: DatabaseRepository<User>,
    customer_repository: DatabaseRepository<Customer>
}

pub trait RepositoriesModuleExt {
    type UserRepo: UserRepository;
    type CustomerRepo: CustomerRepository;

    fn user_repository(&self) -> &Self::UserRepo;
    fn customer_repository(&self) -> &Self::CustomerRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type UserRepo = DatabaseRepository<User>;
    type CustomerRepo = DatabaseRepository<Customer>;

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }

    fn customer_repository(&self) -> &Self::CustomerRepo {
        &self.customer_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let user_repository = DatabaseRepository::new(db.clone());
        let customer_repository = DatabaseRepository::new(db.clone());

        Self {
            user_repository,
            customer_repository,
        }
    }
}