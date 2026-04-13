# Revised Technical Plan: InventivAgents (Rust)

## Global Architecture: Hexagonal Multi-Tenant
Separation of concerns between **Domain** (Core), **Infrastructure** (DB, Auth, MCP), and **Interfaces** (API, CLI).

## Key System Components
1.  **Identity & RBAC Service (IAM)**: Handles multi-tenant User/Group/Role mapping with SSO (OIDC/SAML) support.
2.  **Skill Orchestrator (MCP)**: Loads skills and communicates with MCP servers.
3.  **Observability & Telemetry Engine**: Persists Audit, Telemetry, and Metrics to the DB with TraceID propagation.
4.  **Marketplace Subscription Logic**: Handles cross-tenant subscriptions and billing tracking.
5.  **Coding Sandbox Manager**: Orchestrates secure containers for build and deployment.

## Tech Stack
- **Database**: PostgreSQL with `sqlx` + `pgvector` + **RLS Policies**.
- **Observability**: `tracing` crate with custom subscriber for DB persistence.
- **i18n**: `fluent-rs` with multilingual support (EN, FR, AR).
- **Frontend Telemetry**: API endpoint to receive and log frontend traces.

## Development Roadmap (TDD Milestones)

### Phase 0: Foundations & Infrastructure
- [ ] Docker environment (Postgres, Redis).
- [ ] SQL Migrations (001: Identity, 002: Observability).
- [ ] Hexagonal Boilerplate & Centralized Error Handling.

### Milestone 1: Identity & RBAC (Current)
- [ ] **TDD Unit Tests**: Domain structs for Organization, User, Roles.
- [ ] **TDD Integration Tests**: Database RLS verification.
- [ ] **SSO/OIDC Implementation**: Google/GitHub login flows.
- [ ] **i18n Implementation**: Multi-locale support for API rejections.

### Milestone 2: Observability & Telemetry
- [ ] **TDD Unit Tests**: Logging middleware and TraceID generator.
- [ ] **Persisted Tracing**: Backend logs sent to `telemetry_logs` table.
- [ ] **Frontend Bridge**: Secure endpoint to accept FE logs.

### Milestone 3: Skills & Marketplace
- [ ] Define MCP-compatible "Skill" entity.
- [ ] Subscription logic for cross-tenant sharing.

### Milestone 4: Coding & Sandboxing
- [ ] Docker container orchestration for isolated builds.
- [ ] Git-compatible source control service.

## Security Controls
- Database Row Level Security (RLS) as the primary safety net.
- Argon2 hashing for email/pass auth.
- JWT-based authorization with TraceID headers.
