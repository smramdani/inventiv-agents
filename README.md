# InventivAgents

**Enterprise-Ready Open Source Agentic AI Platform (v0.1.2)**

InventivAgents is a high-performance, secure, and user-friendly platform designed for SMEs to deploy, manage, and scale AI agents within their organization. Built with **Rust**, it prioritizes safety, modularity, and control.

---

## 🚀 Purpose & Vision

Many SMEs want to adopt AI but struggle with security, data isolation, and complexity. InventivAgents provides a "Safe & Friendly" environment where:
- **Organizations** have total control over their data and costs.
- **Agents** are isolated by strict Database Row Level Security (RLS).
- **Skills** can be easily created, used, and even shared in a B2B marketplace.
- **Coding** tasks are executed in secure, virtualized sandboxes.

## ✨ Core Features

- **Multi-Tenancy**: Built-in isolation for organizations using Postgres RLS.
- **Identity & RBAC**: Advanced role management (Owner, Admin, User) with SSO (Google/GitHub/SAML) support.
- **Registry (M3)**: Admin/Owner REST API for LLM providers, MCP/native skills, agents, and agent–skill links (`/org/providers`, `/org/skills`, `/org/agents`).
- **Internationalization**: Native support for English, French, and Arabic.
- **Observability**: Systematic logging, metrics, and telemetry for full execution traceability.
- **Marketplace Ready**: Foundation for cross-organization model and skill sharing.

## Repository layout (monorepo)

| Path | Role |
|------|------|
| **`backend/`** | API Rust (`Cargo.toml`, `src/`, `tests/`, `migrations/`). |
| **`frontend/`** | Cockpit Vite + React (**M5a**) — `npm run dev` ou `make fe-dev` ; voir `frontend/README.md`. |
| **Racine** | Spec Kit (`specify/`, `.specify/`), Docker Compose, `Makefile`, `scripts/`, `README`, `CHANGELOG`. |

Les commandes **`make`** / **`./scripts/dev/dev.sh`** exécutent **cargo** dans `backend/` automatiquement. **`./scripts/dev/with-env.sh cargo …`** en fait autant lorsque la première commande est `cargo`.

**Cockpit (front)** : depuis la racine, `make fe-install` puis `make fe-dev` (API sur `8080` par défaut). CORS côté API : variable **`INVENTIV_CORS_ORIGINS`** (voir `.env.example`) ; le front peut fixer **`VITE_API_BASE`** dans `frontend/.env.local`. Périmètre **M5a** (chat éphémère, sans sessions persistées) vs **M5b** : voir **`specify/spec.md` §5–7** et **`specify/tasks/005_milestone_5.md`**.

## 🛠 Tech Stack

- **Backend**: Rust (Axum, Tokio, SQLx)
- **Database**: PostgreSQL (pgvector for RAG)
- **Security**: JWT Auth + Argon2 + Postgres RLS
- **Infrastructure**: Docker & Redis
- **Licensing**: AGPL-3.0

## 📦 Installation & Setup

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- **Postgres**: either [Docker Desktop](https://www.docker.com/get-started) + Compose **or** a local / managed PostgreSQL (**pgvector** required for migrations) plus **`psql`** on your `PATH` (e.g. macOS: `brew install libpq`)
- [GitHub CLI](https://cli.github.com/) (optional, for contributions)

### Local Development

1. **Clone the repository**:
   ```bash
   git clone https://github.com/smramdani/inventiv-agents.git
   cd inventiv-agents
   ```

2. **Environment** (Compose + app read variables from a project `.env`; Compose also interpolates `${VARS}` in `docker-compose.yml`):
   ```bash
   cp .env.example .env
   # Edit .env: set POSTGRES_PASSWORD and JWT_SECRET for anything beyond local-only use.
   ```

3. **Start infrastructure** (PostgreSQL **pgvector** + Redis, healthchecks, named volumes).  
   Requires **Docker Compose v2** (Docker Desktop includes it). From the repo root:
   ```bash
   docker compose up -d --wait db redis
   docker compose ps
   ```
   The `--wait` flag blocks until **healthchecks** pass. If your Compose version is older and rejects `--wait`, run `docker compose up -d db redis` and wait until `docker compose ps` shows `healthy` for `db` (and `redis`).

   **Without Docker:** run PostgreSQL locally (same port as `POSTGRES_PORT` in `.env`, typically `5432`), ensure extensions **uuid-ossp** and **vector** are available, and align `POSTGRES_USER` / `POSTGRES_PASSWORD` / `POSTGRES_DB` with your instance. Install the client **`psql`**. Then **`make ready`** probes the TCP port and applies migrations via `psql` (or set **`MIGRATE_DATABASE_URL`** to a superuser URL). The API does not require Redis yet; Compose still starts Redis for future use.

4. **Apply database migrations** (ordered `backend/migrations/*.sql`; **Docker** exec or **host `psql`** — see `./scripts/db/apply-migrations.sh`):
   ```bash
   ./scripts/db/apply-migrations.sh
   ```
   On an **already-initialized** database the script exits with a hint; wipe local volumes and reapply with:
   ```bash
   ./scripts/db/reset-local-db.sh
   ```
   Equivalent manual one-liner per file:
   ```bash
   docker compose exec -T db psql -U inventiv_user -d inventiv_agents -v ON_ERROR_STOP=1 < backend/migrations/001_initial_schema_with_rls.sql
   # … repeat for 002, 003, 004, 005
   ```

5. **Run the API** (loads `.env` via `dotenv` in `main`):
   ```bash
   ./scripts/dev/dev.sh run
   ```
   Équivalent manuel depuis la racine : `set -a && source .env && set +a && cd backend && cargo run`.

6. **Run tests** (integration tests use `DATABASE_URL` from `.env`, defaulting to `127.0.0.1:5432` and the `inventiv_app` role).  
   DB integration tests use a **shared named lock** (`serial_test`) so `cargo test` stays safe against one Docker Postgres:
   ```bash
   ./scripts/dev/dev.sh test
   ```
   Ou : `set -a && source .env && set +a && ./scripts/dev/with-env.sh cargo test`.

7. **One-shot local check** (starts Compose, migrates, runs `cargo test` + release build):
   ```bash
   ./scripts/dev/test-local-full.sh
   ```
   Uses `DOCKER_COMPOSE` if set (default `docker compose`).

#### Docker CLI not found (macOS, Cursor, or `make`)

Docker Desktop installs `docker` under **`/Applications/Docker.app/Contents/Resources/bin`**, which is often **not** on `PATH` for GUI-launched terminals, **Cursor** tasks, or **`make`**. This repo’s scripts (`scripts/dev/lib.sh`) **prepend** that path (and `/usr/local/bin`, `/opt/homebrew/bin`) before any `docker` / `docker compose` call. If commands still fail: start **Docker Desktop** until it reports *running*, open a **new** terminal, run **`make doctor`**. Override manually with **`INVENTIV_DOCKER_BIN`** in `.env` (directory containing the `docker` executable). CI and regressions: **`make verify-bootstrap`** runs a stubbed CLI test with a stripped `PATH` (no daemon required).

#### Repeatable dev commands (`dev.sh` + `Makefile`)

Run **`make help`** for a full lifecycle guide, simple verbs (`build`, `release`, `start`, `stop`, `delete`, …), low-level targets, and deploy stub notes. For **when to run manual or full-stack tests** during milestones (especially M4), see `specify/testing-checkpoints.md`.

Use one entry point so every machine runs the same sequence (Docker up, optional migrations, `.env` for `cargo`):

| Simple verb | What it does |
|-------------|----------------|
| `make build` | Debug `cargo build` (with `.env` loaded). |
| `make release` | `cargo build --release`; optional `TAG=v1.0.0` prints a git tag hint. |
| `make start` | Same as `make ready` — DB/Redis up + best-effort migrations. |
| `make stop` | Same as `make down` — stop containers (keeps volumes). |
| `make delete` | Same as `make reset` — **destroys** local DB volume, then migrates. |
| `make test-unit` | Same as `make test-lib` — unit tests only, no Docker. |
| `make fmt` / `make lint` | Format / Clippy only. |
| `make clean` | `cargo clean` dans `backend/`. |
| `make deploy-staging` / `make deploy-prod` | Stubs (`REF=…`, default `latest`); wire to your CI/CD. |

| Goal | Script | Make (equivalent) |
|------|--------|-------------------|
| Sanity check (Docker, Compose, `.env`, Postgres) | `./scripts/dev/dev.sh doctor` | `make doctor` |
| Start DB + Redis | `./scripts/dev/dev.sh up` | `make up` |
| Stop stack | `./scripts/dev/dev.sh down` | `make down` |
| Apply migrations (strict; exit 2 if schema already exists) | `./scripts/dev/dev.sh migrate` | `make migrate` |
| Wipe local volumes and re-migrate | `./scripts/dev/dev.sh reset` | `make reset` |
| Up + best-effort migrate | `./scripts/dev/dev.sh ready` | `make ready` |
| Integration + unit tests | `./scripts/dev/dev.sh test` | `make test` |
| Unit tests only (loads `.env`; no Docker) | `./scripts/dev/dev.sh test-lib` | `make test-lib` |
| Run API (debug) | `./scripts/dev/dev.sh run` | `make run` |
| Run API (release) | `./scripts/dev/dev.sh run-rel` | `make run-rel` |
| fmt + clippy + test | `./scripts/dev/dev.sh check` | `make check` |
| Full pipeline (strict migrate + test + release build) | `./scripts/dev/dev.sh full` | `make full` |
| Docker PATH bootstrap self-test (stub CLI) | `./scripts/dev/verify-docker-bootstrap.sh` | `make verify-bootstrap` |
| M4a SSE smoke (needs running API + `M4A_LLM_API_KEY`) | `./scripts/dev/m4a-mvp-smoke.sh` | `make m4a-smoke` |

Pass arguments through to `cargo` / the binary where supported, for example:

```bash
./scripts/dev/dev.sh test -- --test agents_api
./scripts/dev/dev.sh run -- --help
make run ARGS='-- --help'
make cargo ARGS='test --lib'
```

Load `.env` for any command: `./scripts/dev/with-env.sh cargo clippy --all-targets`.

**MVP engine (no MCP / no tools)** — validation checklist: `specify/mvp-engine-validation.md` (Spec Kit **M4a**). **`make check`** runs the full test suite when Docker **or** host Postgres (TCP) is available; if neither is reachable, it falls back to **library unit tests only** (same scope as `make check-local`). Use **`make check-local`** to force that path without probing Docker/Postgres.

**M4b — MCP (Phase 4, in progress)** — HTTP JSON-RPC client: `inventivagents::infrastructure::mcp::McpHttpJsonRpcClient` implements `McpInvocationPort` (`tools/list`, `tools/call`) against an MCP skill **`endpoint_url`**. Domain helpers: `validate_mcp_invoke_request`, `select_unique_tool_name`. Not yet wired to the public HTTP agent stream (Phase 6). Spec: `specify/tasks/004_milestone_4.md`. Code: `backend/src/infrastructure/mcp/`.

### Docker troubleshooting (local)

- **`Cannot connect to the Docker daemon`**: start Docker Desktop (macOS/Windows) or `sudo systemctl start docker` on Linux.
- **Port `5432` already in use**: set `POSTGRES_PORT=5433` (or another free port) in `.env` and set `DATABASE_URL` to the **same** host port (e.g. `@127.0.0.1:5433/`).
- **Healthcheck never turns healthy**: inspect logs with `docker compose logs db --tail 100`; first boot can take longer on slow disks (`start_period` is 30s for Postgres).
- **Compose vs `docker-compose`**: this repo targets the plugin form `docker compose`; legacy `docker-compose` may work but is not tested.
- **`apply-migrations.sh` exits with “already has schema”**: run `./scripts/db/reset-local-db.sh` once (destroys local Docker volumes for this project), then migrate again.

### Staging / production (e.g. Scaleway)

- Use **Scaleway Managed PostgreSQL** (or Serverless SQL) with TLS (`sslmode=require` in `DATABASE_URL`), private network attachment where possible, and secrets in **Scaleway Secret Manager** (or equivalent), not in the image.
- Run the **same migration sequence** as local using a DB role that can `CREATE EXTENSION`, `CREATE ROLE`, and apply RLS policies (see `backend/migrations/001_initial_schema_with_rls.sql`).
- Point `DATABASE_URL` at the **`inventiv_app`**-style application user created by your migration pipeline; keep a separate superuser only for operations.
- For containers on Scaleway **Kubernetes** or **Serverless Containers**, inject `DATABASE_URL` and `JWT_SECRET` from the secret store; do not bake credentials into the image.

## 📖 Usage

### Register an Organization
```bash
curl -X POST http://localhost:8080/org/register \
  -H "Content-Type: application/json" \
  -d '{"name": "My Company", "admin_email": "admin@company.ai", "locale": "en_US"}'
```

### Login & Authentication
```bash
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "admin@company.ai"}'
```

### Register an LLM provider (Admin or Owner JWT)
```bash
curl -X POST http://localhost:8080/org/providers \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <token_from_login>" \
  -d '{"name": "OVH AI","base_url": "https://api.ovh.com"}'
```

### Agent completion stream (SSE — Milestone 4)

`POST /org/agents/<agent_id>/complete/stream` returns **`text/event-stream`**. Send a JSON body with `message`, `model`, and optional `max_tokens`. Optional header **`X-Trace-ID`** (UUID) is echoed on the response and included in the first SSE `meta` event; if omitted, the server generates one.

```bash
curl -N -X POST "http://localhost:8080/org/agents/<AGENT_UUID>/complete/stream" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <token>" \
  -H "X-Trace-ID: 00000000-0000-4000-8000-000000000001" \
  -d '{"message":"Hello","model":"gpt-4o-mini","max_tokens":256}'
```

SSE events (in order on success): `meta` → `delta` → `usage` → `done`. See doc comments on `post_agent_complete_stream` in `backend/src/api/handlers/engine.rs` for the exact JSON shape per event.

### M4a MVP smoke (Spec Kit gate)

With **Postgres migrated** and the API running (`make run` in another terminal), exercise register → login → provider (with key) → agent → SSE in one command:

```bash
export M4A_LLM_API_KEY="sk-..."   # test key only; never commit
make m4a-smoke
```

Defaults: `M4A_API_BASE=http://127.0.0.1:8080`, `M4A_LLM_BASE_URL=https://api.openai.com`, `M4A_LLM_MODEL=gpt-4o-mini`. See `specify/mvp-engine-validation.md` for the full checklist and sign-off.

## 🤝 Contribution

This project is Open Source under the **AGPL-3.0** license. We welcome contributions!
- Follow the **Clean Code** principles defined in the `specify/constitution.md`.
- Ensure **100% TDD coverage** for new business logic.
- Document all changes in the `CHANGELOG.md`.

---

**Version**: 0.1.2 | **License**: AGPL-3.0 | **Status**: M3 registry + **M4a** (LLM + SSE) + **M5a** (cockpit v1 — **current**); **M5b** (sessions §5) next. **M4b** (MCP in product loop, persistence, orchestration) follows M5; HTTP MCP client in `backend/src/infrastructure/mcp/` is library-only until then. Layout: `backend/` (API), `frontend/` (UI). Local dev: Docker PATH bootstrap, host Postgres fallback, `make verify-bootstrap`, `make m4a-smoke`. Roadmap: `specify/plan.md`; tasks: `specify/tasks/005_milestone_5.md`, `specify/tasks/004_milestone_4.md`.
