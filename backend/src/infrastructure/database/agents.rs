use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::domain::agents::agent::Agent;
use crate::domain::agents::provider::LlmProvider;
use crate::domain::agents::skill::{Skill, SkillType};
use crate::error::{AppError, AppResult};
use crate::infrastructure::database::DatabasePool;

#[derive(Debug, sqlx::FromRow)]
struct LlmProviderRow {
    id: Uuid,
    organization_id: Uuid,
    name: String,
    base_url: String,
    is_active: bool,
}

impl From<LlmProviderRow> for LlmProvider {
    fn from(r: LlmProviderRow) -> Self {
        Self {
            id: r.id,
            organization_id: r.organization_id,
            name: r.name,
            base_url: r.base_url,
            is_active: r.is_active,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct LlmProviderWithKeyRow {
    id: Uuid,
    organization_id: Uuid,
    name: String,
    base_url: String,
    api_key_encrypted: Option<String>,
    is_active: bool,
}

impl LlmProviderWithKeyRow {
    fn into_provider(self) -> LlmProvider {
        LlmProvider {
            id: self.id,
            organization_id: self.organization_id,
            name: self.name,
            base_url: self.base_url,
            is_active: self.is_active,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct AgentRow {
    id: Uuid,
    organization_id: Uuid,
    llm_provider_id: Option<Uuid>,
    name: String,
    mission: String,
    persona: Option<String>,
    is_active: bool,
}

impl From<AgentRow> for Agent {
    fn from(r: AgentRow) -> Self {
        Self {
            id: r.id,
            organization_id: r.organization_id,
            llm_provider_id: r.llm_provider_id,
            name: r.name,
            mission: r.mission,
            persona: r.persona,
            is_active: r.is_active,
        }
    }
}

pub struct AgentsRepository;

impl AgentsRepository {
    fn skill_type_sql(t: SkillType) -> &'static str {
        match t {
            SkillType::MCP => "MCP",
            SkillType::Native => "Native",
        }
    }

    pub async fn insert_llm_provider(
        tx: &mut Transaction<'_, Postgres>,
        org_id: Uuid,
        provider: &LlmProvider,
        api_key_encrypted: Option<&str>,
    ) -> AppResult<()> {
        DatabasePool::set_rls_context(tx, org_id).await?;

        sqlx::query(
            r#"INSERT INTO llm_providers (id, organization_id, name, base_url, api_key_encrypted, is_active)
               VALUES ($1, $2, $3, $4, $5, $6)"#,
        )
        .bind(provider.id)
        .bind(org_id)
        .bind(&provider.name)
        .bind(&provider.base_url)
        .bind(api_key_encrypted)
        .bind(provider.is_active)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn get_agent_by_id(
        tx: &mut Transaction<'_, Postgres>,
        org_id: Uuid,
        agent_id: Uuid,
    ) -> AppResult<Option<Agent>> {
        DatabasePool::set_rls_context(tx, org_id).await?;

        let row = sqlx::query_as::<_, AgentRow>(
            r#"SELECT id, organization_id, llm_provider_id, name, mission, persona, is_active
               FROM agents WHERE id = $1"#,
        )
        .bind(agent_id)
        .fetch_optional(&mut **tx)
        .await?;

        Ok(row.map(Into::into))
    }

    /// Returns provider row plus stored key material (`api_key_encrypted` column; plaintext until KMS).
    pub async fn get_llm_provider_with_key(
        tx: &mut Transaction<'_, Postgres>,
        org_id: Uuid,
        provider_id: Uuid,
    ) -> AppResult<Option<(LlmProvider, Option<String>)>> {
        DatabasePool::set_rls_context(tx, org_id).await?;

        let row = sqlx::query_as::<_, LlmProviderWithKeyRow>(
            r#"SELECT id, organization_id, name, base_url, api_key_encrypted, is_active
               FROM llm_providers WHERE id = $1"#,
        )
        .bind(provider_id)
        .fetch_optional(&mut **tx)
        .await?;

        Ok(row.map(|r| {
            let key = r.api_key_encrypted.clone();
            (r.into_provider(), key)
        }))
    }

    pub async fn list_llm_providers(
        tx: &mut Transaction<'_, Postgres>,
        org_id: Uuid,
    ) -> AppResult<Vec<LlmProvider>> {
        DatabasePool::set_rls_context(tx, org_id).await?;

        let rows: Vec<LlmProviderRow> = sqlx::query_as(
            r#"SELECT id, organization_id, name, base_url, is_active
               FROM llm_providers
               ORDER BY created_at"#,
        )
        .fetch_all(&mut **tx)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn insert_skill(
        tx: &mut Transaction<'_, Postgres>,
        org_id: Uuid,
        skill: &Skill,
    ) -> AppResult<()> {
        DatabasePool::set_rls_context(tx, org_id).await?;

        let type_str = Self::skill_type_sql(skill.skill_type);

        sqlx::query(
            r#"INSERT INTO skills (id, organization_id, name, description, type, endpoint_url, configuration, is_active)
               VALUES ($1, $2, $3, $4, $5::skill_type, $6, $7, $8)"#,
        )
        .bind(skill.id)
        .bind(org_id)
        .bind(&skill.name)
        .bind(&skill.description)
        .bind(type_str)
        .bind(&skill.endpoint_url)
        .bind(&skill.configuration)
        .bind(skill.is_active)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn list_skills(
        tx: &mut Transaction<'_, Postgres>,
        org_id: Uuid,
    ) -> AppResult<Vec<Skill>> {
        DatabasePool::set_rls_context(tx, org_id).await?;

        let rows: Vec<SkillRow> = sqlx::query_as(
            r#"SELECT id, organization_id, name, description, type::text AS skill_type, endpoint_url, configuration, is_active
               FROM skills
               ORDER BY created_at"#,
        )
        .fetch_all(&mut **tx)
        .await?;

        let mut out = Vec::with_capacity(rows.len());
        for row in rows {
            let skill = row
                .into_skill()
                .map_err(|e| AppError::Validation(e.to_string()))?;
            out.push(skill);
        }
        Ok(out)
    }

    pub async fn insert_agent(
        tx: &mut Transaction<'_, Postgres>,
        org_id: Uuid,
        agent: &Agent,
    ) -> AppResult<()> {
        DatabasePool::set_rls_context(tx, org_id).await?;

        sqlx::query(
            r#"INSERT INTO agents (id, organization_id, llm_provider_id, name, mission, persona, is_active)
               VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
        )
        .bind(agent.id)
        .bind(org_id)
        .bind(agent.llm_provider_id)
        .bind(&agent.name)
        .bind(&agent.mission)
        .bind(&agent.persona)
        .bind(agent.is_active)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn list_agents(
        tx: &mut Transaction<'_, Postgres>,
        org_id: Uuid,
    ) -> AppResult<Vec<Agent>> {
        DatabasePool::set_rls_context(tx, org_id).await?;

        let rows: Vec<AgentRow> = sqlx::query_as(
            r#"SELECT id, organization_id, llm_provider_id, name, mission, persona, is_active
               FROM agents
               ORDER BY created_at"#,
        )
        .fetch_all(&mut **tx)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn link_agent_skill(
        tx: &mut Transaction<'_, Postgres>,
        org_id: Uuid,
        agent_id: Uuid,
        skill_id: Uuid,
    ) -> AppResult<()> {
        DatabasePool::set_rls_context(tx, org_id).await?;

        sqlx::query(
            r#"INSERT INTO agent_skills (agent_id, skill_id) VALUES ($1, $2)
               ON CONFLICT DO NOTHING"#,
        )
        .bind(agent_id)
        .bind(skill_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}

/// Row shape for listing skills (enum returned as text from SQL).
#[derive(Debug, sqlx::FromRow)]
pub(crate) struct SkillRow {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub skill_type: String,
    pub endpoint_url: Option<String>,
    pub configuration: serde_json::Value,
    pub is_active: bool,
}

impl SkillRow {
    pub fn into_skill(self) -> anyhow::Result<Skill> {
        let skill_type = match self.skill_type.as_str() {
            "MCP" => SkillType::MCP,
            "Native" => SkillType::Native,
            other => anyhow::bail!("Unknown skill type: {other}"),
        };
        Ok(Skill {
            id: self.id,
            organization_id: self.organization_id,
            name: self.name,
            description: self.description,
            skill_type,
            endpoint_url: self.endpoint_url,
            configuration: self.configuration,
            is_active: self.is_active,
        })
    }
}
