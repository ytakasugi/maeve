use std::sync::Arc;

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tracing::error;

use crate::{
    model::customer::{JsonCreateCustomer, JsonCustomerView},
    module::{Modules, ModulesExt},
};

#[tracing::instrument(skip(modules))]
pub async fn customer_view(
    Path(id): Path<String>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.customer_usecase().find_customer(id).await;

    match res {
        Ok(view) => view
            .map(|view| {
                let json: JsonCustomerView = view.into();
                (StatusCode::OK, Json(json))
            })
            .ok_or(StatusCode::NOT_FOUND),
        Err(err) => {
            error!("Unexpected error: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[tracing::instrument(skip(modules))]
pub async fn create_customer(
    Json(payload): Json<JsonCreateCustomer>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules
        .customer_usecase()
        .create_customer(payload.into())
        .await;

    res.map(|_| StatusCode::CREATED).map_err(|err| {
        error!("Unexpected error: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
