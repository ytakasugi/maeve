use anyhow::Ok;
use derive_new::new;
use maeve_kernel::model::user::NewUser;
use maeve_kernel::model::Id;

#[derive(new)]
pub struct CreateUser {
    pub user_name: String,
    pub password_hash: String,
    pub user_role: String,
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