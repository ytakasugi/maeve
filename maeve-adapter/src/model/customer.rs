use sqlx::FromRow;

use maeve_kernel::model::customer::Customer;

#[derive(FromRow)]
pub struct CustomerTable {
    pub id: String,
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
            customer.name,
            customer.zip_code,
            customer.address,
            customer.phone,
        ))
    }
}