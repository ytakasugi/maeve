use sqlx::FromRow;

use maeve_kernel::model::user::{User, CreateUser};

#[derive(FromRow)]
pub struct UserTable {
    pub id: i32,
    pub user_name: String,
    pub password_hash: String,
    pub user_role: String,
}

impl TryFrom<UserTable> for User {
    type Error = anyhow::Error;

    fn try_from(user: UserTable) -> Result<Self, Self::Error> {
        Ok(
            User::new(
                user.id,
                user.user_name,
                user.password_hash,
                user.user_role,
            )
        )
    }
}

impl TryFrom<UserTable> for CreateUser {
    type Error = anyhow::Error;

    fn try_from(user: UserTable) -> Result<Self, Self::Error> {
        Ok(
            CreateUser::new(
                user.id,
                user.user_name,
                user.password_hash,
                user.user_role,
            )
        )
    }
}