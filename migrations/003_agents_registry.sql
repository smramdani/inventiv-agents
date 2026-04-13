-- 003_agents_registry.sql

-- Enum Types
CREATE TYPE skill_type AS ENUM ('MCP', 'Native');

-- 1. LLM Providers (Sovereign Managed APIs)
CREATE TABLE llm_providers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    base_url TEXT NOT NULL,
    api_key_encrypted TEXT, -- Encrypted key
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE llm_providers ENABLE ROW LEVEL SECURITY;
ALTER TABLE llm_providers FORCE ROW LEVEL SECURITY;

-- 2. Skills (Atomic units of capability)
CREATE TABLE skills (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    type skill_type NOT NULL DEFAULT 'MCP',
    endpoint_url TEXT, -- URL for MCP SSE or Native hook
    configuration JSONB NOT NULL DEFAULT '{}',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE skills ENABLE ROW LEVEL SECURITY;
ALTER TABLE skills FORCE ROW LEVEL SECURITY;

-- 3. Agents (Mission-oriented entities)
CREATE TABLE agents (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    llm_provider_id UUID REFERENCES llm_providers(id) ON DELETE SET NULL,
    name TEXT NOT NULL,
    mission TEXT NOT NULL, -- The "What"
    persona TEXT, -- The "Who" (Style, Tone)
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE agents ENABLE ROW LEVEL SECURITY;
ALTER TABLE agents FORCE ROW LEVEL SECURITY;

-- 4. Agent Skills Junction
CREATE TABLE agent_skills (
    agent_id UUID NOT NULL REFERENCES agents(id) ON DELETE CASCADE,
    skill_id UUID NOT NULL REFERENCES skills(id) ON DELETE CASCADE,
    PRIMARY KEY (agent_id, skill_id)
);

ALTER TABLE agent_skills ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_skills FORCE ROW LEVEL SECURITY;

-- RLS POLICIES (Consistent with app.current_org_id)

CREATE POLICY llm_provider_isolation_policy ON llm_providers
    FOR ALL USING (organization_id::text = current_setting('app.current_org_id', true));

CREATE POLICY skill_isolation_policy ON skills
    FOR ALL USING (organization_id::text = current_setting('app.current_org_id', true));

CREATE POLICY agent_isolation_policy ON agents
    FOR ALL USING (organization_id::text = current_setting('app.current_org_id', true));

CREATE POLICY agent_skill_isolation_policy ON agent_skills
    FOR ALL USING (
        EXISTS (
            SELECT 1 FROM agents a 
            WHERE a.id = agent_skills.agent_id 
            AND a.organization_id::text = current_setting('app.current_org_id', true)
        )
    );

-- Grant privileges to the restricted app user
GRANT ALL PRIVILEGES ON TABLE llm_providers, skills, agents, agent_skills TO inventiv_app;
