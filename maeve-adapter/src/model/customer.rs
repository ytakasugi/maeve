use sqlx::FromRow;

use maeve_kernel::model::customer::{Customer, NewCustomer};

#[derive(FromRow)]
pub struct CustomerTable {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub zip_code: String,
    pub address: String,
    pub phone: String,
}

impl TryFrom<CustomerTable> for Customer {
    type Error = anyhow::Error;

    fn try_from(customer: CustomerTable) -> Result<Self, Self::Error> {
        Ok(Customer::new(
            customer.id.try_into()?,
            customer.user_id.try_into()?,
            customer.name,
            customer.zip_code,
            customer.address,
            customer.phone,
        ))
    }
}

impl TryFrom<NewCustomer> for CustomerTable {
    type Error = anyhow::Error;

    fn try_from(customer: NewCustomer) -> Result<Self, Self::Error> {
        Ok(CustomerTable {
            id: customer.id.value.to_string(),
            user_id: customer.user_id.value.to_string(),
            name: customer.name,
            zip_code: customer.zip_code,
            address: customer.address,
            phone: customer.phone,
        })
    }
}
