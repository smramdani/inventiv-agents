use dotenvy::dotenv;
use inventivagents::api::Server;
use inventivagents::infrastructure::database::DatabasePool;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting InventivAgents Backend (AGPL-3.0)");

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = DatabasePool::connect(&db_url).await?;

    let server = Server::new(db_pool);
    server.run("0.0.0.0:8080").await?;

    Ok(())
}
