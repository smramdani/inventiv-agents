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

2. **Start Infrastructure**:
   ```bash
   docker compose up -d
   ```

3. **Apply Migrations** (in order):
   ```bash
   docker exec -i inventivagents-db-1 psql -U inventiv_user -d inventiv_agents < migrations/001_initial_schema_with_rls.sql
   docker exec -i inventivagents-db-1 psql -U inventiv_user -d inventiv_agents < migrations/002_observability_schema.sql
   docker exec -i inventivagents-db-1 psql -U inventiv_user -d inventiv_agents < migrations/003_agents_registry.sql
   docker exec -i inventivagents-db-1 psql -U inventiv_user -d inventiv_agents < migrations/004_agents_registry_indexes_and_grants.sql
   ```

4. **Run the Backend**:
   ```bash
   cargo run
   ```

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
