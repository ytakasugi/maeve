use crate::{
    module::Modules,
    routes::{
        user::{
            user_view,
            create_user
        }
    }
};
use axum::{
    routing::{get, post},
    extract::Extension, Router,
};

use std::{net::SocketAddr, sync::Arc};

use crate::util::logger;

pub async fn startup(modules: Arc<Modules>) {
    logger::init();

    let user_router = Router::new()
        .route("/:id", get(user_view))
        .route("/", post(create_user));

    let app = Router::new()
        .nest("/users", user_router)
        .layer(Extension(modules));
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    tracing::info!("Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch!"))
}