use std::marker::PhantomData;

use derive_new::new;

use crate::persistence::database::Db;

pub mod customer;
pub mod user;
pub mod user_detail;

#[derive(new)]
pub struct DatabaseRepository<T> {
    pool: Db,
    _marker: PhantomData<T>,
}
