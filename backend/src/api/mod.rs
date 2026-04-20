use std::time::Duration;

use axum::http::header::{self, AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use axum::middleware::from_fn;
use axum::routing::{get, post, Router};
use axum::Json;
use serde::Serialize;
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::api::middleware::auth::AuthenticatedUser;
use crate::api::middleware::observability::trace_id_middleware;
use crate::domain::identity::user::UserRole;
use crate::infrastructure::database::DatabasePool;

pub mod handlers;
pub mod middleware;

pub struct Server {
    db: DatabasePool,
}

#[derive(Serialize)]
pub struct WhoAmIResponse {
    pub user_id: String,
    pub org_id: String,
    pub role: UserRole,
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
        app_router(self.db.clone())
    }
}

/// HTTP entrypoint for tests and [`Server`].
pub fn app_router(db: DatabasePool) -> Router {
    Router::new()
        .route(
            "/org/register",
            post(handlers::identity::register_organization),
        )
        .route("/auth/login", post(handlers::identity::login))
        .route("/auth/whoami", get(whoami_handler))
        .route(
            "/telemetry/frontend",
            post(handlers::telemetry::handle_frontend_telemetry),
        )
        .route("/org/users", post(handlers::identity::invite_user))
        .route("/org/groups", post(handlers::identity::create_group))
        .route(
            "/org/providers",
            get(handlers::agents::list_providers).post(handlers::agents::create_provider),
        )
        .route(
            "/org/skills",
            get(handlers::agents::list_skills).post(handlers::agents::create_skill),
        )
        .route(
            "/org/agents",
            get(handlers::agents::list_agents).post(handlers::agents::create_agent),
        )
        .route(
            "/org/agents/:agent_id/skills/:skill_id",
            post(handlers::agents::link_agent_skill),
        )
        .route(
            "/org/agents/:agent_id/complete/stream",
            post(handlers::engine::post_agent_complete_stream),
        )
        .layer(from_fn(trace_id_middleware))
        .layer(inventiv_cors_layer())
        .with_state(db)
}

fn inventiv_cors_layer() -> CorsLayer {
    let raw = std::env::var("INVENTIV_CORS_ORIGINS")
        .unwrap_or_else(|_| "http://127.0.0.1:5173,http://localhost:5173".to_string());
    let origins: Vec<HeaderValue> = raw
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse().ok())
        .collect();
    let allow_origin = if origins.is_empty() {
        AllowOrigin::list(vec!["http://127.0.0.1:5173"
            .parse()
            .expect("static cors origin")])
    } else {
        AllowOrigin::list(origins)
    };
    CorsLayer::new()
        .allow_origin(allow_origin)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([
            AUTHORIZATION,
            CONTENT_TYPE,
            header::HeaderName::from_static("x-trace-id"),
        ])
        .expose_headers([header::HeaderName::from_static("x-trace-id")])
        .max_age(Duration::from_secs(3600))
}

async fn whoami_handler(AuthenticatedUser(claims): AuthenticatedUser) -> Json<WhoAmIResponse> {
    Json(WhoAmIResponse {
        user_id: claims.sub.to_string(),
        org_id: claims.org_id.to_string(),
        role: claims.role,
    })
}
