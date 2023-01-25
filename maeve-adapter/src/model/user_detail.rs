use sqlx::FromRow;

use maeve_kernel::model::user_detail::UserDetail;

#[derive(FromRow)]
pub struct UserDetailTable {
    pub user_id: String,
    pub user_name: String,
    pub customer_id: String,
    pub customer_name: String,
    pub zip_code: String,
    pub address: String,
    pub phone: String,
}

impl TryFrom<UserDetailTable> for UserDetail {
    type Error = anyhow::Error;

    fn try_from(detail: UserDetailTable) -> Result<Self, Self::Error> {
        Ok(UserDetail::new(
            detail.user_id.try_into()?,
            detail.user_name,
            detail.customer_id.try_into()?,
            detail.customer_name,
            detail.zip_code,
            detail.address,
            detail.phone,
        ))
    }
}
