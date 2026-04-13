use crate::api::Server;
use crate::infrastructure::database::DatabasePool;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenvy::dotenv;

mod api;
mod domain;
mod infrastructure;
mod i18n;
mod error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 0. Load environment
    dotenv().ok();

    // 1. Initialize Tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting InventivAgents Backend (AGPL-3.0)");

    // 2. Initialize Database
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = DatabasePool::connect(&db_url).await?;

    // 3. Start Axum Server
    let server = Server::new(db_pool);
    server.run("0.0.0.0:8080").await?;

    Ok(())
}
