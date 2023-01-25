use std::sync::Arc;

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tracing::error;

use crate::{
    model::user_detail::JsonUserDetailList,
    module::{Modules, ModulesExt},
};

#[tracing::instrument(skip(modules))]
pub async fn user_detail_view(
    Path(id): Path<String>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.user_detail_usecase().find_user_detail(id).await;

    match res {
        Ok(detail_list) => match detail_list {
            Some(detail) => {
                let details = detail.into_iter().map(|d| d.into()).collect();
                let json: JsonUserDetailList = JsonUserDetailList::new(details);
                Ok((StatusCode::OK, Json(json)))
            }
            None => {
                let json: JsonUserDetailList = JsonUserDetailList::new(vec![]);
                Ok((StatusCode::OK, Json(json)))
            }
        },
        Err(err) => {
            error!("Unexpected error: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
