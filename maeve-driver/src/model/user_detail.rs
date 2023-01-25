use maeve_app::model::user_detail::UserDetailView;
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonUserDetailView {
    user_id: String,
    user_name: String,
    customer_id: String,
    customer_name: String,
    zip_code: String,
    address: String,
    phone: String,
}

#[derive(Serialize)]
pub struct JsonUserDetailList {
    pub list: Vec<JsonUserDetailView>,
}

impl From<UserDetailView> for JsonUserDetailView {
    fn from(detail: UserDetailView) -> Self {
        JsonUserDetailView {
            user_id: detail.user_id,
            user_name: detail.user_name,
            customer_id: detail.customer_id,
            customer_name: detail.customer_name,
            zip_code: detail.zip_code,
            address: detail.address,
            phone: detail.phone,
        }
    }
}

impl JsonUserDetailList {
    pub fn new(list: Vec<JsonUserDetailView>) -> Self {
        Self { list }
    }
}
