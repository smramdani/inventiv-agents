# InventivAgents Cockpit (M5a)

Vite + React + TypeScript SPA: authentication (`/auth/login`, `/auth/whoami`), organization registration (`/org/register`), **Registry** (Owner/Admin: providers, skills, agents), **ephemeral** SSE chat to `POST /org/agents/:id/complete/stream`, last **`usage`** event display.

**Scope**: **M5a** only — no persisted server-side sessions. **M5b** (sessions, group sharing) and full **US.5** on persisted `metrics` are defined in **`../specify/spec.md` §5–7** and **`../specify/tasks/005_milestone_5.md`**.

## Prerequisites

- Rust API running (`make run` from monorepo root).
- CORS: the API allows `http://127.0.0.1:5173` and `http://localhost:5173` by default. Set **`INVENTIV_CORS_ORIGINS`** (comma-separated) in the API `.env` for other origins.

## Environment

Create `frontend/.env.local` (gitignored) if needed:

```bash
# API base URL (no trailing slash)
VITE_API_BASE=http://127.0.0.1:8080
```

## Commands

```bash
npm install
npm run dev      # http://127.0.0.1:5173
npm run build
npm run lint     # tsc --noEmit
```

From repo root: `make fe-install`, `make fe-dev`, `make fe-build`, `make fe-lint`.

JWT is stored in **sessionStorage** (`inventiv_jwt`) and cleared when the tab is closed.
