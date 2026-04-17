mod common;

use dotenvy::dotenv;
use inventivagents::domain::agents::agent::Agent;
use inventivagents::domain::agents::provider::LlmProvider;
use inventivagents::domain::agents::skill::{Skill, SkillType};
use inventivagents::infrastructure::database::agents::AgentsRepository;
use serial_test::serial;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

#[tokio::test]
#[serial(integration_db)]
async fn test_agents_registry_rls_isolation() -> anyhow::Result<()> {
    dotenv().ok();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&common::app_database_url())
        .await?;

    let org_a = Uuid::new_v4();
    let org_b = Uuid::new_v4();
    common::insert_org(&pool, org_a, "Org Agents A").await?;
    common::insert_org(&pool, org_b, "Org Agents B").await?;

    let provider = LlmProvider::new(org_a, "OVH", "https://api.ovh.com")?;

    let mut tx = pool.begin().await?;
    AgentsRepository::insert_llm_provider(&mut tx, org_a, &provider, None).await?;
    tx.commit().await?;

    let mut tx_b = pool.begin().await?;
    let list_b = AgentsRepository::list_llm_providers(&mut tx_b, org_b).await?;
    tx_b.commit().await?;
    assert!(
        list_b.is_empty(),
        "Org B must not see Org A providers under RLS"
    );

    let mut tx_a = pool.begin().await?;
    let list_a = AgentsRepository::list_llm_providers(&mut tx_a, org_a).await?;
    tx_a.commit().await?;
    assert_eq!(list_a.len(), 1);
    assert_eq!(list_a[0].name, "OVH");

    Ok(())
}

#[tokio::test]
#[serial(integration_db)]
async fn test_admin_can_create_agent_with_multiple_skills() -> anyhow::Result<()> {
    dotenv().ok();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&common::app_database_url())
        .await?;

    let org_id = Uuid::new_v4();
    common::insert_org(&pool, org_id, "Org HR").await?;

    let provider = LlmProvider::new(org_id, "LLM", "https://api.openai.com")?;
    let skill_a = Skill::new(
        org_id,
        "Policy KB",
        SkillType::MCP,
        Some("https://mcp.internal/policy".into()),
    )?;
    let skill_b = Skill::new(org_id, "SQL", SkillType::Native, None)?;
    let agent = Agent::new(
        org_id,
        "HR Agent",
        "Help employees with HR questions",
        Some(provider.id),
    )?;

    let mut tx = pool.begin().await?;
    AgentsRepository::insert_llm_provider(&mut tx, org_id, &provider, None).await?;
    AgentsRepository::insert_skill(&mut tx, org_id, &skill_a).await?;
    AgentsRepository::insert_skill(&mut tx, org_id, &skill_b).await?;
    AgentsRepository::insert_agent(&mut tx, org_id, &agent).await?;
    AgentsRepository::link_agent_skill(&mut tx, org_id, agent.id, skill_a.id).await?;
    AgentsRepository::link_agent_skill(&mut tx, org_id, agent.id, skill_b.id).await?;
    tx.commit().await?;

    let mut tx2 = pool.begin().await?;
    let skills = AgentsRepository::list_skills(&mut tx2, org_id).await?;
    tx2.commit().await?;
    assert_eq!(skills.len(), 2);

    let mut tx3 = pool.begin().await?;
    let agents = AgentsRepository::list_agents(&mut tx3, org_id).await?;
    tx3.commit().await?;
    assert_eq!(agents.len(), 1);
    assert_eq!(agents[0].name, "HR Agent");

    Ok(())
}
