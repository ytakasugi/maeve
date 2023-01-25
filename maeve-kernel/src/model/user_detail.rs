use derive_new::new;

use super::Id;
use crate::model::customer::Customer;
use crate::model::user::User;

#[derive(new, Debug)]
pub struct UserDetail {
    pub user_id: Id<User>,
    pub user_name: String,
    pub customer_id: Id<Customer>,
    pub customer_name: String,
    pub zip_code: String,
    pub address: String,
    pub phone: String,
}
