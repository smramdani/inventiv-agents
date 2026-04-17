-- 004_agents_registry_indexes_and_grants.sql
-- Tenant-scoped lookups and enum usage for the restricted app role.

CREATE INDEX IF NOT EXISTS idx_llm_providers_organization_id ON llm_providers(organization_id);
CREATE INDEX IF NOT EXISTS idx_skills_organization_id ON skills(organization_id);
CREATE INDEX IF NOT EXISTS idx_agents_organization_id ON agents(organization_id);
CREATE INDEX IF NOT EXISTS idx_agents_llm_provider_id ON agents(llm_provider_id);

GRANT USAGE ON TYPE skill_type TO inventiv_app;
