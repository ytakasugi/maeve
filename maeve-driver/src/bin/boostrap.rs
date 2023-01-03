use std::sync::Arc;

use maeve_driver::{
    module::Modules,
    startup::{init_app, startup},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_app();

    let modules = Modules::new().await;
    startup(Arc::new(modules)).await;

    Ok(())
}