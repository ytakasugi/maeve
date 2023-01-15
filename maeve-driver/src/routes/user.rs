use std::sync::Arc;

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tracing::error;

use crate::{
    model::user::{JsonCreateUser, JsonUserView},
    module::{Modules, ModulesExt},
};

#[tracing::instrument(skip(modules))]
pub async fn user_view(
    Path(id): Path<String>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.user_usecase().find_user(id).await;

    match res {
        Ok(view) => view
            .map(|view| {
                let json: JsonUserView = view.into();
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
pub async fn create_user(
    Json(payload): Json<JsonCreateUser>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.user_usecase().create_user(payload.into()).await;

    res.map(|_| StatusCode::CREATED).map_err(|err| {
        error!("Unexpected error: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip(modules))]
pub async fn delete_user(
    Path(id): Path<String>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    modules
        .user_usecase()
        .delete_user(id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|err| {
            error!("Unexpected error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
