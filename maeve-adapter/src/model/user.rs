use sqlx::FromRow;

use maeve_kernel::model::user::{User, NewUser};

#[derive(FromRow)]
pub struct UserTable {
    pub id: String,
    pub user_name: String,
    pub password_hash: String,
    pub user_role: String,
}

impl TryFrom<UserTable> for User {
    type Error = anyhow::Error;

    fn try_from(user: UserTable) -> Result<Self, Self::Error> {
        Ok(
            User::new(
                user.id.try_into()?,
                user.user_name,
                user.password_hash,
                user.user_role,
            )
        )
    }
}

impl TryFrom<NewUser> for UserTable {
    type Error = anyhow::Error;

    fn try_from(user: NewUser) -> Result<Self, Self::Error> {
        Ok(
            UserTable {
                id: user.id.value.to_string(),
                user_name: user.user_name,
                password_hash: user.password_hash,
                user_role: user.user_role,
            }
        )
    }
}