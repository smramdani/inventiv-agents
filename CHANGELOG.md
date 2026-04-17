# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- **Repository layout**: Rust API crate, SQL migrations, and integration tests moved under **`backend/`**; **`frontend/`** added for the M5 cockpit (README placeholder). Root keeps Spec Kit, Compose, `Makefile`, and `scripts/`. Dev scripts run **cargo** from `backend/` (`inventiv_cargo`, `with-env.sh cargo …`); migrations applied from **`backend/migrations/*.sql`**. Docs and `.gitignore` / `.dockerignore` updated.
- **Roadmap (Spec Kit)**: **M5** (Sovereign Cockpit / front-end) is the **current** delivery priority; **M4b** Phases **4–6** (MCP in the live reasoning loop, persisted runs/metrics, full orchestration) are **scheduled after M5**. Aligned docs: `specify/plan.md`, `specify/spec.md` §7, `specify/tasks/004_milestone_4.md`, `specify/tasks/005_milestone_5.md`, `specify/mvp-engine-validation.md`, `specify/testing-checkpoints.md`, `README.md`.

### Added
- **M4b Phase 4 (Spec Kit)**: `src/infrastructure/mcp/` — `McpHttpJsonRpcClient` implements `McpInvocationPort` (JSON-RPC `tools/list` / `tools/call` over HTTP POST, 30s timeout, 2 MiB response cap). Domain: `validate_mcp_invoke_request`, `select_unique_tool_name`. Wiremock contract tests in `http_client.rs`. Docs: `README.md`, `specify/plan.md`, `specify/tasks/004_milestone_4.md`, `mvp-engine-validation.md`, `testing-checkpoints.md`.

## [0.1.2] - 2026-04-14

### Changed
- **Docker CLI on macOS**: `scripts/dev/lib.sh` bootstraps `PATH` with Docker Desktop’s **`Docker.app/Contents/Resources/bin`** (and Homebrew locations) before `docker` / `docker compose` checks, so **`make`**, Cursor, and non-login shells find the CLI. Optional **`INVENTIV_DOCKER_BIN`** in `.env`; **`apply-migrations.sh`** / **`reset-local-db.sh`** source the same library. README troubleshooting; **`make doctor`** calls the bootstrap explicitly.
- **Local dev without Docker**: `make ready`, `make test`, `make run`, `make check`, and `make full` use **`inventiv_ensure_local_database`** — start Compose when Docker works, otherwise require an open Postgres port (`POSTGRES_HOST` / `POSTGRES_PORT`). **`./scripts/db/apply-migrations.sh`** falls back to host **`psql`** using `MIGRATE_DATABASE_URL` or `POSTGRES_*`. **`make doctor`** passes when either Compose or host Postgres + `psql` works; **`make down`** no-ops if Docker is missing. **`reset-local-db`** still requires Docker (volume wipe).
- **Startup log**: `main` logs the crate **SemVer** at API boot (`CARGO_PKG_VERSION`, constitution version visibility).

### Added
- **`make verify-bootstrap`**: `scripts/dev/verify-docker-bootstrap.sh` exercises `INVENTIV_DOCKER_BIN` + `inventiv_has_docker` with a stripped `PATH` (no real Docker daemon).
- **M4a MVP smoke**: `scripts/dev/m4a-mvp-smoke.sh` and `make m4a-smoke` / `./scripts/dev/dev.sh m4a-smoke` — headless register → login → provider+key → agent → SSE validation (requires running API and `M4A_LLM_API_KEY`). Documented in README, `specify/mvp-engine-validation.md`, and `specify/testing-checkpoints.md`.

## [0.1.1] - 2026-04-13

### Fixed
- **Integration tests / RLS**: `tests/common::app_database_url()` now connects as `inventiv_app` even when `DATABASE_URL` points at the local superuser (`inventiv_user`), which bypasses RLS (`BYPASSRLS`). Host, port, database name, and query string are preserved; use `INVENTIV_TEST_DATABASE_URL` for a fully explicit override (documented in `.env.example`).

### Changed
- **Dev tooling**: `make check` / `dev.sh check` runs `fmt` + `clippy`, then full `cargo test` if Docker/Compose works, else falls back to `cargo test --lib` with a notice. `check-local` forces lib-only. `lib.sh`: `inventiv_has_docker`. Docs: `make help`, README, `mvp-engine-validation`, `testing-checkpoints`.
- **Roadmap (Spec Kit)**: `specify/plan.md` splits **M4a** (MVP engine: LLM + SSE, no tools / no MCP) vs **M4b** (MCP, persistence, full reasoning loop — deferred). `specify/spec.md` §7 and `specify/tasks/004_milestone_4.md` aligned; Phases 4–6 marked deferred until **`specify/mvp-engine-validation.md`** sign-off. **`specify/testing-checkpoints.md`** prioritizes M4a MVP gates.
- **`/org/register`**: set `app.current_org_id` in the same transaction before inserts so registration succeeds under RLS (`inventiv_app`).
- **`/auth/login`**: use `lookup_user_for_login` instead of a direct `users` select blocked by RLS without org context.
- **README**: local development documents `make help`, simple Makefile verbs (`build`, `release`, `start`, `stop`, …), and deploy stub targets; manual migration list includes `005`.
- **Docker local stack**: Compose file top-level `name: inventivagents`; Postgres healthcheck uses `pg_isready` on `127.0.0.1` with correct `$$` env expansion; longer `start_period`; scripts use `DOCKER_COMPOSE` and `pg_isready -h 127.0.0.1`; `test-local-full.sh` tries `compose up --wait` then falls back; `apply-migrations.sh` refuses re-run if schema exists; `reset-local-db.sh` for clean volumes; README Docker troubleshooting and `--wait` docs; `.dockerignore` for future images.
- **Integration tests**: `serial_test` with shared `integration_db` lock across `tests/*.rs` so parallel `cargo test` does not corrupt shared Postgres.
- **`.gitignore`**: ignore `.env` (use committed `.env.example` as template).

### Added
- **`specify/mvp-engine-validation.md`**: Checklist and sign-off for **M4a** (engine without tools/MCP): automated `make check`, integration coverage, optional manual SSE with real test provider; explicit deferral of MCP / run persistence until M4b.
- **Milestone 4 (API, Phase 3 — Spec Kit T4.7–T4.9)**: `POST /org/agents/:agent_id/complete/stream` returns SSE (`meta`, `delta`, `usage`, `error`, `done`); `trace_id` in structured logs and in `meta` / `X-Trace-ID`; handler `src/api/handlers/engine.rs`; deps `async-stream`, `futures-core`; integration test `tests/sse_agent_stream_integration.rs`.
- **Migration `005_login_lookup_and_register_rls.sql`**: `lookup_user_for_login(email)` (`SECURITY DEFINER`) so `/auth/login` works under RLS with role `inventiv_app`.
- **HTTP integration tests**: `tests/identity_http.rs` (`/org/register`, `/auth/login`, `/auth/whoami`, 401 smoke for `/org/users`, `/org/groups`, `/telemetry/frontend`); extended `tests/agents_api.rs` (GET `/org/providers` auth, full registry list/create/link flow). Helpers `insert_admin_user` / `admin_bearer_token` in `tests/common`.
- **Integration tests (M4 LLM resolution)**: `tests/llm_resolve_integration.rs` — DB-seeded provider + agent → `openai_compatible_client_for_agent` → wiremock completion; negative path when agent has no provider. Shared `tests/common::insert_org`; explicit `sqlx` + `anyhow` in `[dev-dependencies]` for integration crates.
- **Milestone 4 (infra, Phase 2)**: `src/infrastructure/llm/` — `OpenAiCompatibleClient` (`POST /v1/chat/completions`, `LlmCompletionPort`), `openai_compatible_client_for_agent` resolver; `AgentsRepository::{get_agent_by_id,get_llm_provider_with_key}`; domain `TokenUsage`; dev-dependency `wiremock` for client tests.
- **Spec Kit**: `specify/testing-checkpoints.md` — when to run manual/full-stack tests per milestone (esp. M4 SSE/MCP and real LLM timing).
- **Milestone 4 (domain, Phase 1)**: `src/domain/engine/` — `ReasoningPhase` / `TransitionInput` / `EngineError` with tests; `LlmCompletionPort` + request/response types; `McpInvocationPort` + tool list/invoke types; dependency `async-trait` for port traits.
- **Spec Kit — Milestone 4**: Task list `specify/tasks/004_milestone_4.md` (Agentic Engine: LLM ports, SSE API, MCP client, reasoning loop, run/metrics persistence, XIV gates); `specify/plan.md` links M4 to that file; `specify/spec.md` §7 summarizes M4 technical scope and US.4 linkage.
- **Constitution 1.4.0**: Principle **XV** (immutable release artifact, dev→staging→prod promotion, externalized configuration and secrets, CI/CD pipeline obligations); **XII** and **XIV** cross-references; **Development Workflow** step 7 for CD.
- **Local & cloud Postgres layout**: `docker-compose.yml` with healthchecks, stable service/container names, named volumes, and `inventiv` bridge network; `.env.example` for Compose + app variables; `scripts/db/apply-migrations.sh`; README section for local workflow and Scaleway-oriented staging/production notes.
- **Integration test config**: `tests/common/mod.rs`; optional `INVENTIV_TEST_DATABASE_URL`; superuser `DATABASE_URL` values are rewritten to `inventiv_app` for RLS-correct tests.
- **Governance & SDD**: Constitution **1.3.0** with **XIV — Definition of Done by Layer** (vertical slice, database, domain, API, front-end when applicable, cross-cutting); extended **XII** and **Development Workflow** for Spec Kit checklist/analyze gates. Mirror at `.specify/memory/constitution.md` for `/speckit.plan` and `/speckit.analyze`.
- **Spec Kit templates**: Optional layer tags on generated tasks; checklist template requires a Constitution XIV layer section when relevant.
- **Milestone 3 tasks**: Layer definition-of-done gate section in `specify/tasks/003_milestone_3.md` (front-end marked N/A for M3 per plan).
- **Agents registry (Milestone 3)**: Migrations `003_agents_registry.sql` and `004_agents_registry_indexes_and_grants.sql`; domain `provider` / `skill` / `agent`; `AgentsRepository` with RLS on every path; Admin/Owner HTTP routes under `/org/providers`, `/org/skills`, `/org/agents`, and agent–skill linking; library crate `src/lib.rs` and `app_router` for integration tests; integration tests `agents_registry_rls`, `agents_api`, and corrected `identity_rls` transaction scoping for `set_config`.
- **Local dev tooling**: `scripts/dev/lib.sh` (shared Docker/env helpers), `scripts/dev/with-env.sh` (run any command with `.env` loaded), `scripts/dev/dev.sh` (doctor, up/down, migrate/reset/ready, test, run, check, full), root `Makefile` with **simple verbs** (`build`, `release`, `start`, `stop`, `delete`, `fmt`, `lint`, `clean`, …), **`make help`** (`scripts/dev/make-help.txt` lifecycle + deploy stubs), deploy placeholder targets, and `test-local-full.sh` refactored to reuse the shared library.

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
