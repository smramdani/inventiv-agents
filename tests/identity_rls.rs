use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

async fn insert_org_with_context(
    pool: &sqlx::PgPool,
    org_id: Uuid,
    name: &str,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::query("SELECT set_config('app.current_org_id', $1, true)")
        .bind(org_id.to_string())
        .execute(&mut *tx)
        .await?;

    sqlx::query("INSERT INTO organizations (id, name, default_locale) VALUES ($1, $2, $3)")
        .bind(org_id)
        .bind(name)
        .bind("en_US")
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(())
}

#[tokio::test]
async fn test_rls_isolation_between_orgs() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url =
        "postgres://inventiv_app:inventiv_app_password@localhost:5432/inventiv_agents";

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    let org_a_id = Uuid::new_v4();
    let org_b_id = Uuid::new_v4();

    insert_org_with_context(&pool, org_a_id, "Org A").await?;
    insert_org_with_context(&pool, org_b_id, "Org B").await?;

    let email_a = format!("user_a_{}@example.com", Uuid::new_v4());
    let email_b = format!("user_b_{}@example.com", Uuid::new_v4());

    {
        let mut tx = pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_org_id', $1, true)")
            .bind(org_a_id.to_string())
            .execute(&mut *tx)
            .await?;
        sqlx::query(
            "INSERT INTO users (organization_id, email, role) VALUES ($1, $2, $3::user_role)",
        )
        .bind(org_a_id)
        .bind(&email_a)
        .bind("Admin")
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
    }

    {
        let mut tx = pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_org_id', $1, true)")
            .bind(org_b_id.to_string())
            .execute(&mut *tx)
            .await?;
        sqlx::query(
            "INSERT INTO users (organization_id, email, role) VALUES ($1, $2, $3::user_role)",
        )
        .bind(org_b_id)
        .bind(&email_b)
        .bind("User")
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
    }

    let mut tx = pool.begin().await?;
    sqlx::query("SELECT set_config('app.current_org_id', $1, true)")
        .bind(org_a_id.to_string())
        .execute(&mut *tx)
        .await?;

    let rows: Vec<(String,)> = sqlx::query_as("SELECT email FROM users")
        .fetch_all(&mut *tx)
        .await?;
    tx.commit().await?;

    assert_eq!(rows.len(), 1, "RLS should filter out users from Org B");
    assert_eq!(rows[0].0, email_a);

    Ok(())
}
