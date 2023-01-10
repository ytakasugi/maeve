use std::marker::PhantomData;

use derive_new::new;

use crate::persistence::database::Db;

pub mod user;
pub mod customer;

#[derive(new)]
pub struct DatabaseRepository<T> {
    pool: Db,
    _marker: PhantomData<T>
}