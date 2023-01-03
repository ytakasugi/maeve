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
use dotenv::dotenv;
use std::{net::SocketAddr, sync::Arc};

pub async fn startup(modules: Arc<Modules>) {
    init_logger();

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

fn init_logger() {
    dotenv().ok();

    // ログ出力を設定する
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true) 
        .compact();
    
        tracing_subscriber::fmt()
        .event_format(format)
        .init();
}