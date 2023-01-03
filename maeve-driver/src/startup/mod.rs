use crate::{
    module::Modules,
    routes::{
        user::create_user
    }
};
use axum::{
    routing::{post},
    extract::Extension, Router,
};
use std::{net::SocketAddr, sync::Arc};

pub async fn startup(modules: Arc<Modules>) {
    let user_router = Router::new()
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

pub fn init_app() {
    tracing_subscriber::fmt::init();
}