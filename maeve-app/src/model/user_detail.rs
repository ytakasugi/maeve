use maeve_kernel::model::user_detail::UserDetail;
pub struct UserDetailView {
    pub user_id: String,
    pub user_name: String,
    pub customer_id: String,
    pub customer_name: String,
    pub zip_code: String,
    pub address: String,
    pub phone: String,
}

impl UserDetailView {
    pub fn new(detail: UserDetail) -> Self {
        Self {
            user_id: detail.user_id.value.to_string(),
            user_name: detail.user_name,
            customer_id: detail.customer_id.value.to_string(),
            customer_name: detail.customer_name,
            zip_code: detail.zip_code,
            address: detail.address,
            phone: detail.phone,
        }
    }
}

impl From<UserDetail> for UserDetailView {
    fn from(detail: UserDetail) -> Self {
        Self {
            user_id: detail.user_id.value.to_string(),
            user_name: detail.user_name,
            customer_id: detail.customer_id.value.to_string(),
            customer_name: detail.customer_name,
            zip_code: detail.zip_code,
            address: detail.address,
            phone: detail.phone,
        }
    }
}
