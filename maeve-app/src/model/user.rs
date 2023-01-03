use derive_new::new;
use maeve_kernel::model::{
    user::{User, NewUser},
    Id
};

pub struct UserView {
    pub id: String,
    pub user_name: String,
    pub password_hash: String,
    pub user_role: String,
}

#[derive(new)]
pub struct CreateUser {
    pub user_name: String,
    pub password_hash: String,
    pub user_role: String,
}

impl UserView {
    pub fn new(user: User) -> Self {
        Self {
            id: user.id.value.to_string(),
            user_name: user.user_name,
            password_hash: user.password_hash,
            user_role: user.user_role,
        }
    }
}

impl TryFrom<CreateUser> for NewUser {
    type Error = anyhow::Error;

    fn try_from(c: CreateUser) -> Result<Self, Self::Error> {
        let user_id = Id::gen();
        Ok(
            NewUser::new(
                user_id,
                c.user_name,
                c.password_hash,
                c.user_role
            )
        )
    }
}