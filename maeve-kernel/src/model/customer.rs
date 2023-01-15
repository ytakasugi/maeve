use derive_new::new;

use super::Id;

#[derive(new, Debug, PartialEq, Eq)]
pub struct Customer {
    pub id: Id<Customer>,
    pub user_id: String,
    pub name: String,
    pub zip_code: String,
    pub address: String,
    pub phone: String,
}

#[derive(new, Debug)]
pub struct NewCustomer {
    pub id: Id<Customer>,
    pub user_id: String,
    pub name: String,
    pub zip_code: String,
    pub address: String,
    pub phone: String,
}
