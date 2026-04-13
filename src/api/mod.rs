use axum::{routing::post, routing::get, Router};
use axum::middleware::from_fn;
use crate::infrastructure::database::DatabasePool;
use crate::api::middleware::auth::AuthenticatedUser;
use crate::api::middleware::observability::trace_id_middleware;
use axum::Json;
use serde::Serialize;

pub mod handlers;
pub mod middleware;

pub struct Server {
    db: DatabasePool,
}

#[derive(Serialize)]
pub struct WhoAmIResponse {
    pub user_id: String,
    pub org_id: String,
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
            .route("/auth/login", post(handlers::identity::login))
            .route("/auth/whoami", get(whoami_handler))
            .route("/telemetry/frontend", post(handlers::telemetry::handle_frontend_telemetry))
            .route("/org/users", post(handlers::identity::invite_user))
            .route("/org/groups", post(handlers::identity::create_group))
            .layer(from_fn(trace_id_middleware))
            .with_state(self.db.clone())
    }
}

async fn whoami_handler(
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Json<WhoAmIResponse> {
    Json(WhoAmIResponse {
        user_id: claims.sub.to_string(),
        org_id: claims.org_id.to_string(),
    })
}
