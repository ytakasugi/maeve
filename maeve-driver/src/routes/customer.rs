use std::sync::Arc;

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tracing::error;

use crate::{
    model::customer::JsonCreateCustomer,
    module::{Modules, ModulesExt},
};

#[tracing::instrument(skip(modules))]
pub async fn create_customer(
    Path(id): Path<String>,
    Json(payload): Json<JsonCreateCustomer>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules
        .customer_usecase()
        .create_customer(id, payload.into())
        .await;

    res.map(|_| StatusCode::CREATED).map_err(|err| {
        error!("Unexpected error: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
