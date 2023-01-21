use derive_new::new;
use maeve_kernel::model::customer::{Customer, NewCustomer};
use maeve_kernel::model::Id;

pub struct CustomerView {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub zip_code: String,
    pub address: String,
    pub phone: String,
}

#[derive(new)]
pub struct CreateCustomer {
    pub user_id: String,
    pub name: String,
    pub zip_code: String,
    pub address: String,
    pub phone: String,
}

impl CustomerView {
    pub fn new(customer: Customer) -> Self {
        Self {
            id: customer.id.value.to_string(),
            user_id: customer.user_id.value.to_string(),
            name: customer.name,
            zip_code: customer.zip_code,
            address: customer.address,
            phone: customer.phone,
        }
    }
}

impl TryFrom<CreateCustomer> for NewCustomer {
    type Error = anyhow::Error;

    fn try_from(c: CreateCustomer) -> Result<Self, Self::Error> {
        let customer_id = Id::gen();
        Ok(NewCustomer::new(
            customer_id,
            c.user_id.try_into()?,
            c.name,
            c.zip_code,
            c.address,
            c.phone,
        ))
    }
}
