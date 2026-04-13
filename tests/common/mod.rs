//! Shared helpers for integration tests (`mod common;` from each `tests/*.rs` crate).

pub fn app_database_url() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://inventiv_app:inventiv_app_password@127.0.0.1:5432/inventiv_agents".into()
    })
}
