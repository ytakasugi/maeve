use maeve_kernel::model::customer::Customer;

pub struct CustomerView {
    pub id: String,
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
