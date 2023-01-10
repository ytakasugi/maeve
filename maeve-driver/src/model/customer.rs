use serde::{
    //Serialize,
    Deserialize
};

use maeve_kernel::model::customer::NewCustomer;

#[derive(Deserialize, Debug)]
pub struct JsonCreateCustomer {
    name: String,
    email: String,
    address: String,
    phone: String,
}

impl From<JsonCreateCustomer> for NewCustomer {
    fn from(c: JsonCreateCustomer) -> Self {
        NewCustomer { 
            name: c.name,
            email: c.email,
            address: c.address,
            phone: c.phone, 
        }
    }
}