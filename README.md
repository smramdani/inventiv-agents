# InventivAgents

**Enterprise-Ready Open Source Agentic AI Platform (v0.1.0)**

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

## 🛠 Tech Stack

- **Backend**: Rust (Axum, Tokio, SQLx)
- **Database**: PostgreSQL (pgvector for RAG)
- **Security**: JWT Auth + Argon2 + Postgres RLS
- **Infrastructure**: Docker & Redis
- **Licensing**: AGPL-3.0

## 📦 Installation & Setup

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Docker](https://www.docker.com/get-started) & Docker Compose
- [GitHub CLI](https://cli.github.com/) (for contributions)

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

4. **Apply database migrations** (ordered `migrations/*.sql` via the superuser in the container):
   ```bash
   ./scripts/db/apply-migrations.sh
   ```
   On an **already-initialized** database the script exits with a hint; wipe local volumes and reapply with:
   ```bash
   ./scripts/db/reset-local-db.sh
   ```
   Equivalent manual one-liner per file:
   ```bash
   docker compose exec -T db psql -U inventiv_user -d inventiv_agents -v ON_ERROR_STOP=1 < migrations/001_initial_schema_with_rls.sql
   # … repeat for 002, 003, 004
   ```

5. **Run the API** (loads `.env` via `dotenv` in `main`):
   ```bash
   set -a && source .env && set +a   # or: export $(grep -v '^#' .env | xargs)
   cargo run
   ```

6. **Run tests** (integration tests use `DATABASE_URL` from `.env`, defaulting to `127.0.0.1:5432` and the `inventiv_app` role).  
   DB integration tests use a **shared named lock** (`serial_test`) so `cargo test` stays safe against one Docker Postgres:
   ```bash
   set -a && source .env && set +a
   cargo test
   ```

7. **One-shot local check** (starts Compose, migrates, runs `cargo test` + release build):
   ```bash
   ./scripts/dev/test-local-full.sh
   ```
   Uses `DOCKER_COMPOSE` if set (default `docker compose`).

### Docker troubleshooting (local)

- **`Cannot connect to the Docker daemon`**: start Docker Desktop (macOS/Windows) or `sudo systemctl start docker` on Linux.
- **Port `5432` already in use**: set `POSTGRES_PORT=5433` (or another free port) in `.env` and set `DATABASE_URL` to the **same** host port (e.g. `@127.0.0.1:5433/`).
- **Healthcheck never turns healthy**: inspect logs with `docker compose logs db --tail 100`; first boot can take longer on slow disks (`start_period` is 30s for Postgres).
- **Compose vs `docker-compose`**: this repo targets the plugin form `docker compose`; legacy `docker-compose` may work but is not tested.
- **`apply-migrations.sh` exits with “already has schema”**: run `./scripts/db/reset-local-db.sh` once (destroys local Docker volumes for this project), then migrate again.

### Staging / production (e.g. Scaleway)

- Use **Scaleway Managed PostgreSQL** (or Serverless SQL) with TLS (`sslmode=require` in `DATABASE_URL`), private network attachment where possible, and secrets in **Scaleway Secret Manager** (or equivalent), not in the image.
- Run the **same migration sequence** as local using a DB role that can `CREATE EXTENSION`, `CREATE ROLE`, and apply RLS policies (see `migrations/001_initial_schema_with_rls.sql`).
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

## 🤝 Contribution

This project is Open Source under the **AGPL-3.0** license. We welcome contributions!
- Follow the **Clean Code** principles defined in the `specify/constitution.md`.
- Ensure **100% TDD coverage** for new business logic.
- Document all changes in the `CHANGELOG.md`.

---

**Version**: 0.1.0 | **License**: AGPL-3.0 | **Status**: Milestone 3 registry delivered; engine and cockpit next per plan.
