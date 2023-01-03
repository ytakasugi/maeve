use serde::Deserialize;
use maeve_app::model::user::CreateUser;

#[derive(Deserialize, Debug)]
pub struct JsonCreateUser {
    user_name: String,
    password_hash: String,
    user_role: String,
}

impl From<JsonCreateUser> for CreateUser {
    fn from(u: JsonCreateUser) -> Self {
        CreateUser {
            user_name: u.user_name,
            password_hash: u.password_hash,
            user_role: u.user_role,
        }
    }
}