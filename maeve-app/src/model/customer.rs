use derive_new::new;
use maeve_kernel::model::customer::{Customer, NewCustomer};

pub struct CustomerView {
    pub id: String,
    pub name: String,
    pub email: String,
    pub address: String,
    pub phone: String
}

#[derive(new)]
pub struct CreateCustomer {
    pub name: String,
    pub email: String,
    pub address: String,
    pub phone: String
}

impl CustomerView {
    pub fn new(customer: Customer) -> Self {
        Self {
            id: customer.id.value.to_string(),
            name: customer.name,
            email: customer.email,
            address: customer.address,
            phone: customer.phone
        }
    }
}

impl TryFrom<CreateCustomer> for NewCustomer {
    type Error = anyhow::Error;

    fn try_from(c: CreateCustomer) -> Result<Self, Self::Error> {
        Ok(
            NewCustomer::new(
                c.name,
                c.email,
                c.address,
                c.phone
            )
        )
    }
}