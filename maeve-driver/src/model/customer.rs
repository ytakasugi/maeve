use serde::{
    //Serialize,
    Deserialize
};

use maeve_app::model::customer::CreateCustomer;

#[derive(Deserialize, Debug)]
pub struct JsonCreateCustomer {
    name: String,
    email: String,
    address: String,
    phone: String,
}

impl From<JsonCreateCustomer> for CreateCustomer {
    fn from(c: JsonCreateCustomer) -> Self {
        CreateCustomer { 
            name: c.name,
            email: c.email,
            address: c.address,
            phone: c.phone, 
        }
    }
}