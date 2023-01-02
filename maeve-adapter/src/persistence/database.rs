use dotenv::dotenv;
use std::sync::Arc;

use sqlx::PgPool;

#[derive(Clone)]
pub struct Db (pub(crate) Arc<PgPool>);

impl Db {
    pub async fn new() -> Db {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE URL MUST BE SET.");

        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
            .unwrap_or_else(|_| {
            panic!("Failed create connection pool.")
        });

        Db(Arc::new(pool))
    }
}