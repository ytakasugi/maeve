use derive_new::new;

use super::Id;

#[derive(new, Debug, PartialEq, Eq)]
pub struct Customer {
    pub id: Id<Customer>,
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
