use derive_new::new;

#[derive(new, Debug)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub password_hash: String,
    pub user_role: String,
}

#[derive(new, Debug)]
pub struct CreateUser {
    pub id: i32,
    pub user_name: String,
    pub password_hash: String,
    pub user_role: String,
}