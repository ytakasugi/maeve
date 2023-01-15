use derive_new::new;

use super::Id;
use crate::model::user::User;

#[derive(new, Debug, PartialEq, Eq)]
pub struct Customer {
    pub id: Id<User>,
    pub name: String,
    pub zip_code: String,
    pub address: String,
    pub phone: String,
}

#[derive(new, Debug)]
pub struct NewCustomer {
    pub name: String,
    pub zip_code: String,
    pub address: String,
    pub phone: String,
}
