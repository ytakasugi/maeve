use derive_new::new;

use super::Id;

#[derive(new, Debug, PartialEq, Eq)]
pub struct User {
    pub id: Id<User>,
    pub user_name: String,
    pub email: String,
    pub password_hash: String,
    pub user_role: String,
}

#[derive(new, Debug)]
pub struct NewUser {
    pub id: Id<User>,
    pub user_name: String,
    pub email: String,
    pub password_hash: String,
    pub user_role: String,
}
