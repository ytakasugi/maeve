use crate::{
    module::Modules,
    routes::{
        customer::create_customer,
        user::{create_user, delete_user, user_view},
    },
};
use axum::{
    extract::Extension,
    routing::{delete, get, post},
    Router,
};

use std::{net::SocketAddr, sync::Arc};

use crate::util::logger;

pub async fn startup(modules: Arc<Modules>) {
    logger::init();

    let user_router = Router::new()
        .route("/", post(create_user))
        .route("/:id", get(user_view))
        .route("/:id", delete(delete_user));

    let customer_router = Router::new().route("/:id", post(create_customer));

    let app = Router::new()
        .nest("/users", user_router)
        .nest("/customers", customer_router)
        .layer(Extension(modules));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    tracing::info!("Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch!"))
}
