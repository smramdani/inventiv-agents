# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Local & cloud Postgres layout**: `docker-compose.yml` with healthchecks, stable service/container names, named volumes, and `inventiv` bridge network; `.env.example` for Compose + app variables; `scripts/db/apply-migrations.sh`; README section for local workflow and Scaleway-oriented staging/production notes.
- **Integration test config**: `tests/common/mod.rs` and `DATABASE_URL` override for CI/staging databases.
- **Governance & SDD**: Constitution **1.3.0** with **XIV — Definition of Done by Layer** (vertical slice, database, domain, API, front-end when applicable, cross-cutting); extended **XII** and **Development Workflow** for Spec Kit checklist/analyze gates. Mirror at `.specify/memory/constitution.md` for `/speckit.plan` and `/speckit.analyze`.
- **Spec Kit templates**: Optional layer tags on generated tasks; checklist template requires a Constitution XIV layer section when relevant.
- **Milestone 3 tasks**: Layer definition-of-done gate section in `specify/tasks/003_milestone_3.md` (front-end marked N/A for M3 per plan).
- **Agents registry (Milestone 3)**: Migrations `003_agents_registry.sql` and `004_agents_registry_indexes_and_grants.sql`; domain `provider` / `skill` / `agent`; `AgentsRepository` with RLS on every path; Admin/Owner HTTP routes under `/org/providers`, `/org/skills`, `/org/agents`, and agent–skill linking; library crate `src/lib.rs` and `app_router` for integration tests; integration tests `agents_registry_rls`, `agents_api`, and corrected `identity_rls` transaction scoping for `set_config`.

### Changed
- **`.gitignore`**: ignore `.env` (use committed `.env.example` as template).

## [0.1.0] - 2026-04-13

### Added
- **Multi-Tenant Identity & RBAC**:
    - Organization registration and Owner setup.
    - User invitation system (Owner, Admin, User roles).
    - Group management (Create groups within Organization).
    - Secure JWT-based Authentication.
- **Security & Safety**:
    - Hardened PostgreSQL **Row Level Security (RLS)** using `FORCE ROW LEVEL SECURITY`.
    - Dedicated restricted `inventiv_app` database user for execution isolation.
- **Infrastructure**:
    - Modular Hexagonal Architecture in Rust.
    - TDD implementation for Domain and Integration layers.
    - Dockerized Postgres (with pgvector) and Redis.
- **Documentation**:
    - Comprehensive README.md and Project Constitution.
    - Synthetic CHANGELOG initialization.
