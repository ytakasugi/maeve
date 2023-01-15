use serde::Deserialize;

use maeve_app::model::customer::CreateCustomer;

#[derive(Deserialize, Debug)]
pub struct JsonCreateCustomer {
    name: String,
    zip_code: String,
    address: String,
    phone: String,
}

impl From<JsonCreateCustomer> for CreateCustomer {
    fn from(c: JsonCreateCustomer) -> Self {
        CreateCustomer {
            name: c.name,
            zip_code: c.zip_code,
            address: c.address,
            phone: c.phone,
        }
    }
}
