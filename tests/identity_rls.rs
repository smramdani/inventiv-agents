use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;
use dotenvy::dotenv;

#[tokio::test]
async fn test_rls_isolation_between_orgs() -> anyhow::Result<()> {
    dotenv().ok();
    // For RLS testing, we MUST use a user that is NOT the table owner
    let database_url = "postgres://inventiv_app:inventiv_app_password@localhost:5432/inventiv_agents";
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // 1. Create two Organizations
    let org_a_id = Uuid::new_v4();
    let org_b_id = Uuid::new_v4();

    sqlx::query("INSERT INTO organizations (id, name, default_locale) VALUES ($1, $2, $3)")
        .bind(org_a_id)
        .bind("Org A")
        .bind("en_US")
        .execute(&pool)
        .await?;

    sqlx::query("INSERT INTO organizations (id, name, default_locale) VALUES ($1, $2, $3)")
        .bind(org_b_id)
        .bind("Org B")
        .bind("fr_FR")
        .execute(&pool)
        .await?;

    // 2. Create a user for each Org
    let email_a = format!("user_a_{}@example.com", Uuid::new_v4());
    let email_b = format!("user_b_{}@example.com", Uuid::new_v4());

    sqlx::query("INSERT INTO users (organization_id, email, role) VALUES ($1, $2, $3::user_role)")
        .bind(org_a_id)
        .bind(&email_a)
        .bind("Admin")
        .execute(&pool)
        .await?;

    sqlx::query("INSERT INTO users (organization_id, email, role) VALUES ($1, $2, $3::user_role)")
        .bind(org_b_id)
        .bind(&email_b)
        .bind("User")
        .execute(&pool)
        .await?;

    // 3. Set session context to Org A
    // IMPORTANT: This mimics the middleware behavior in production
    sqlx::query("SELECT set_config('app.current_org_id', $1, true)")
        .bind(org_a_id.to_string())
        .execute(&pool)
        .await?;

    // 4. Query users: Should ONLY see Org A's user
    let rows: Vec<(String,)> = sqlx::query_as("SELECT email FROM users")
        .fetch_all(&pool)
        .await?;

    assert_eq!(rows.len(), 1, "RLS should filter out users from Org B");
    assert_eq!(rows[0].0, "user_a@example.com");

    Ok(())
}
