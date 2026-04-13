-- 001_initial_schema_with_rls.sql

-- Extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "vector";

-- Enum Types
CREATE TYPE user_role AS ENUM ('Owner', 'Admin', 'User');
CREATE TYPE group_member_role AS ENUM ('Member', 'Organizer');

-- Tables

-- 1. Organizations
CREATE TABLE organizations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    default_locale TEXT NOT NULL DEFAULT 'en_US',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Enable RLS for Organizations
ALTER TABLE organizations ENABLE ROW LEVEL SECURITY;
ALTER TABLE organizations FORCE ROW LEVEL SECURITY;

-- 2. Users
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT,
    role user_role NOT NULL DEFAULT 'User',
    preferred_locale TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE users ENABLE ROW LEVEL SECURITY;
ALTER TABLE users FORCE ROW LEVEL SECURITY;

-- 3. Groups
CREATE TABLE groups (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE groups ENABLE ROW LEVEL SECURITY;
ALTER TABLE groups FORCE ROW LEVEL SECURITY;

-- 4. Group Memberships
CREATE TABLE group_members (
    group_id UUID NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role group_member_role NOT NULL DEFAULT 'Member',
    PRIMARY KEY (group_id, user_id)
);

ALTER TABLE group_members ENABLE ROW LEVEL SECURITY;
ALTER TABLE group_members FORCE ROW LEVEL SECURITY;

-- 5. OAuth Identities
CREATE TABLE oauth_identities (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,
    provider_user_id TEXT NOT NULL,
    UNIQUE(provider, provider_user_id)
);

ALTER TABLE oauth_identities ENABLE ROW LEVEL SECURITY;
ALTER TABLE oauth_identities FORCE ROW LEVEL SECURITY;

-- Restricted App User
DO $$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'inventiv_app') THEN
        CREATE ROLE inventiv_app WITH LOGIN PASSWORD 'inventiv_app_password';
    END IF;
END
$$;

GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO inventiv_app;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO inventiv_app;
GRANT ALL PRIVILEGES ON SCHEMA public TO inventiv_app;

-- RLS POLICIES (Using safe current_setting)
-- The second parameter 'true' in current_setting ensures it returns NULL instead of an error if not set.

CREATE POLICY organization_isolation_policy ON organizations
    FOR ALL USING (id::text = current_setting('app.current_org_id', true));

CREATE POLICY user_isolation_policy ON users
    FOR ALL USING (organization_id::text = current_setting('app.current_org_id', true));

CREATE POLICY group_isolation_policy ON groups
    FOR ALL USING (organization_id::text = current_setting('app.current_org_id', true));

CREATE POLICY group_member_isolation_policy ON group_members
    FOR ALL USING (
        EXISTS (
            SELECT 1 FROM groups g 
            WHERE g.id = group_members.group_id 
            AND g.organization_id::text = current_setting('app.current_org_id', true)
        )
    );

CREATE POLICY oauth_identity_isolation_policy ON oauth_identities
    FOR ALL USING (
        EXISTS (
            SELECT 1 FROM users u 
            WHERE u.id = oauth_identities.user_id 
            AND u.organization_id::text = current_setting('app.current_org_id', true)
        )
    );
