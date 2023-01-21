use serde::{Deserialize, Serialize};

use maeve_app::model::customer::{CreateCustomer, CustomerView};

#[derive(Serialize)]
pub struct JsonCustomerView {
    id: String,
    user_id: String,
    name: String,
    zip_code: String,
    address: String,
    phone: String,
}

#[derive(Deserialize, Debug)]
pub struct JsonCreateCustomer {
    user_id: String,
    name: String,
    zip_code: String,
    address: String,
    phone: String,
}

impl From<CustomerView> for JsonCustomerView {
    fn from(c: CustomerView) -> Self {
        JsonCustomerView {
            id: c.id,
            user_id: c.user_id,
            name: c.name,
            zip_code: c.zip_code,
            address: c.address,
            phone: c.phone,
        }
    }
}

impl From<JsonCreateCustomer> for CreateCustomer {
    fn from(c: JsonCreateCustomer) -> Self {
        CreateCustomer {
            user_id: c.user_id,
            name: c.name,
            zip_code: c.zip_code,
            address: c.address,
            phone: c.phone,
        }
    }
}
