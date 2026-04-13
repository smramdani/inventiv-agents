use axum::{routing::post, Router};
use crate::infrastructure::database::DatabasePool;

pub mod handlers;
pub mod middleware;

pub struct Server {
    db: DatabasePool,
}

impl Server {
    pub fn new(db: DatabasePool) -> Self {
        Self { db }
    }

    pub async fn run(self, addr: &str) -> anyhow::Result<()> {
        let app = self.create_router();
        let listener = tokio::net::TcpListener::bind(addr).await?;
        
        tracing::info!("Listening on {}", addr);
        axum::serve(listener, app).await?;
        
        Ok(())
    }

    fn create_router(&self) -> Router {
        Router::new()
            .route("/org/register", post(handlers::identity::register_organization))
            .with_state(self.db.clone())
    }
}
