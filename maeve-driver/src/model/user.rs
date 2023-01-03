use serde::{Serialize, Deserialize};
use maeve_app::model::user::{UserView, CreateUser};

#[derive(Serialize)]
pub struct JsonUserView {
    id: String,
    user_name: String,
    password_hash: String,
    user_role: String,
}

#[derive(Deserialize, Debug)]
pub struct JsonCreateUser {
    user_name: String,
    password_hash: String,
    user_role: String,
}

impl From<UserView> for JsonUserView {
    fn from(u: UserView) -> Self {
        JsonUserView { 
            id: u.id,
            user_name: u.user_name,
            password_hash: u.password_hash,
            user_role: u.user_role, 
        }
    }
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