-- 002_observability_schema.sql

-- Enum Types for Observability
CREATE TYPE log_level AS ENUM ('DEBUG', 'INFO', 'WARN', 'ERROR');
CREATE TYPE log_source AS ENUM ('Backend', 'Frontend', 'Agent');
CREATE TYPE notification_level AS ENUM ('Info', 'Warning', 'Error', 'Critical');

-- 1. Audit Logs (Business Events)
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action TEXT NOT NULL, -- e.g., 'user.created', 'skill.modified'
    resource_type TEXT NOT NULL, -- e.g., 'Organization', 'User', 'Session'
    resource_id UUID,
    metadata JSONB NOT NULL DEFAULT '{}',
    ip_address INET,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE audit_logs ENABLE ROW LEVEL SECURITY;

-- 2. Telemetry Logs (Technical Traces)
-- Designed for high-volume structured logging from FE and BE
CREATE TABLE telemetry_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    trace_id UUID NOT NULL, -- Correlates multiple spans in one request
    span_id UUID,
    parent_span_id UUID,
    level log_level NOT NULL DEFAULT 'INFO',
    source log_source NOT NULL,
    message TEXT NOT NULL,
    context JSONB NOT NULL DEFAULT '{}', -- Arbitrary structured data (args, stack traces)
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE telemetry_logs ENABLE ROW LEVEL SECURITY;
CREATE INDEX idx_telemetry_trace_id ON telemetry_logs(trace_id);

-- 3. Performance & Usage Metrics
CREATE TABLE metrics (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    metric_name TEXT NOT NULL, -- e.g., 'llm.token_usage', 'api.latency_ms'
    metric_value DOUBLE PRECISION NOT NULL,
    labels JSONB NOT NULL DEFAULT '{}', -- e.g., {"model": "gpt-4", "endpoint": "/chat"}
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE metrics ENABLE ROW LEVEL SECURITY;
CREATE INDEX idx_metrics_name_time ON metrics(metric_name, timestamp DESC);

-- 4. Notifications (Actionable Alerts)
CREATE TABLE notifications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE, -- Target user (NULL for all admins)
    level notification_level NOT NULL DEFAULT 'Info',
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    read_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE notifications ENABLE ROW LEVEL SECURITY;

-- RLS POLICIES (Consistent with app.current_org_id)

CREATE POLICY audit_log_isolation_policy ON audit_logs
    FOR ALL USING (organization_id = current_setting('app.current_org_id')::UUID);

CREATE POLICY telemetry_log_isolation_policy ON telemetry_logs
    FOR ALL USING (organization_id = current_setting('app.current_org_id')::UUID);

CREATE POLICY metrics_isolation_policy ON metrics
    FOR ALL USING (organization_id = current_setting('app.current_org_id')::UUID);

CREATE POLICY notification_isolation_policy ON notifications
    FOR ALL USING (organization_id = current_setting('app.current_org_id')::UUID);
