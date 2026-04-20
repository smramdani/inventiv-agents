# Milestone 5: The Sovereign Cockpit (front-end + sessions)

**Status**: **Current** roadmap priority (see `specify/plan.md` §2). **Prerequisites**: M4a engine (SSE + LLM) validated; M3 registry APIs available.

**Deferred until after M5**: **M4b** Phases 4–6 integration in the product sense — MCP/tool **reasoning loop**, run/metrics **persistence**, and **US.2** (MCP in the loop) / full **US.3** agent+tool UX — see `specify/tasks/004_milestone_4.md` Phases 4–6. The HTTP MCP client in `backend/src/infrastructure/mcp/` remains a **library foundation**; wiring into live agent flows waits until **post-M5** per roadmap.

## Purpose

Deliver the first **authenticated web client** (or SPA) so organizations can use the platform without `curl`: org context, LLM provider setup, user invite flows where applicable, **chat sessions** with agents (consuming existing SSE API), and foundations for **usage / cost visibility** (US.5 can start read-only from SSE `usage` + future metrics tables).

## Layer gates (Constitution XIV)

### Front-end `[FE]` — **in scope**

- UX: loading, empty, error, success states; no secrets in client errors.
- Telemetry: hook significant client errors to existing backend telemetry paths where defined (IX).
- Version: display app / API version in UI per constitution X.

### API `[API]`

- Extend or document routes only as needed for the cockpit (session CRUD when added, etc.); reuse M3/M4a endpoints where possible.

### Database `[DB]`

- Session and sharing tables when introduced: **RLS** + **FORCE RLS**; SQL migrations live under **`backend/migrations/`** (ordered, documented).

## Tasks (outline — expand in follow-up PRs)

- [x] **T5.1** `[FE]` Repo layout: **`backend/`** (Rust API) + **`frontend/`** (cockpit) at repo root; shared tooling at root (`Makefile`, `scripts/`, `specify/`). Vite + React app under `frontend/`.
- [x] **T5.2** `[FE]` Auth: login / register flows against existing `/org/register`, `/auth/login`, JWT storage (secure), `/auth/whoami`.
- [x] **T5.3** `[FE]` Admin paths: list/create LLM providers, skills, agents (M3 APIs); role-gated UI.
- [x] **T5.4** `[FE]` User path: start a **session** (or equivalent) calling **`POST /org/agents/:id/complete/stream`** with SSE consumer (EventSource / fetch stream).
- [x] **T5.5** `[FE]` Owner / cost: first **usage** view (even if v1 reads last SSE `usage` client-side only; align later with persisted metrics post-M4b).
- [x] **T5.6** Cross-cutting: README, CHANGELOG, `specify/testing-checkpoints.md` M5 gates.

**Checkpoint**: Manual smoke: two browsers or two users for session sharing when that feature lands; `make check` green for backend; FE lint/build in CI when wired.

**Implemented (v1 cockpit)**:

- **API**: `INVENTIV_CORS_ORIGINS`, **`GET /auth/whoami`** returns **`role`**, **`GET /org/agents`** allowed for any authenticated org member (chat picker for `User`).
- **FE**: Vite + React + TS in `frontend/` — routes `/login`, `/register`, `/`, `/registry`, `/chat`; `make fe-install` / `fe-dev` / `fe-build` / `fe-lint`.
