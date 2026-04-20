# Milestone 5: The Sovereign Cockpit (front-end + sessions)

**Status**: **M5a shipped**; **M5b** and **M4b** remain. Roadmap row: `specify/plan.md` §2. **Prerequisites**: M4a engine (SSE + LLM) validated; M3 registry APIs available.

**Deferred until after M5 (product MCP path)**: **M4b** Phases 4–6 — MCP/tool **reasoning loop**, run/metrics **persistence**, **US.2** / full **US.3** — see `specify/tasks/004_milestone_4.md` Phases 4–6. HTTP MCP client in `backend/src/infrastructure/mcp/` is **library only** until **M4b**.

## Purpose

Deliver the first **authenticated web client** so organizations can use the platform without `curl`: org context, LLM provider setup, user invite flows where applicable, **agent conversations** (SSE against M4a), and foundations for **usage / cost visibility**.

**Spec alignment (`specify/spec.md`)**: **§5** defines full **Sessions** (history, RAG, group sharing). **M5a** deliberately delivers **ephemeral chat** only; **M5b** closes the gap to §5. **§3 / US.5**: **M5a** shows per-turn SSE **`usage`**; persisted **`metrics`** and owner reports land with **M4b** (and later M5b UX on top).

## Layer gates (Constitution XIV)

### Front-end `[FE]`

- **M5a**: Loading / empty / error / success paths on main flows; no secrets in user-visible errors; cockpit + API version in footer (**X**). Telemetry: partial (`whoami` failure path); **M5b+**: systematic significant-error reporting to `/telemetry/frontend` (**IX**).
- **M5b**: Same gates for new session surfaces; a11y and audit export when in scope (see testing checkpoints).

### API `[API]`

- **M5a**: Reuse M3/M4a routes; CORS + `whoami` role + org-scoped list queries documented in README / `.env.example`.
- **M5b**: New session CRUD + RLS as designed; document contracts before merge (**XIV**).

### Database `[DB]`

- **M5b**: Session (and related) tables: **RLS** + **FORCE RLS** where required; ordered SQL under **`backend/migrations/`**.

---

## M5a — shipped (cockpit v1)

Constitution **XII**: tasks below map to **§6–7** (early **US.1** / **US.4** slice, not full **US.5** or §5 sessions).

- [x] **T5.1** `[FE]` Repo layout: **`backend/`** + **`frontend/`**; shared tooling at repo root; Vite + React + TS in `frontend/`.
- [x] **T5.2** `[FE]` Auth: `/org/register`, `/auth/login`, JWT in **sessionStorage**, `/auth/whoami` (with **role**). *Hardening backlog:* httpOnly cookies / refresh tokens (**security**, not M5a).
- [x] **T5.3** `[FE]` Admin: **create + list** providers, skills, agents (M3 APIs); Owner/Admin-gated UI. *Not in M5a:* edit/delete, agent–skill link UI (`POST /org/agents/.../skills/...`), invite user / groups screens.
- [x] **T5.4** `[FE]` **Ephemeral** agent chat: **`POST /org/agents/:id/complete/stream`** via `fetch` + SSE parser (not EventSource). **Not** persisted multi-turn **sessions** (that is **M5b** / §5).
- [x] **T5.5** `[FE]` **Usage (M5a scope):** last SSE **`usage`** for the turn. **Not** full **US.5** / `metrics` dashboard (**M4b** + **M5b**).
- [x] **T5.6** Cross-cutting: README, CHANGELOG, `specify/testing-checkpoints.md` for M5a; **`make fe-install` / `fe-dev` / `fe-build` / `fe-lint`**.

**Checkpoint (M5a)**: Manual smoke: register → login → registry → chat with trace + usage; **`make check`** / **`make fe-lint`** green on touched code.

---

## M5b — planned (sessions + collaboration, §5)

- [ ] **T5.7** `[DB]` Migrations: `sessions` (and related) with **RLS** / **FORCE RLS**; org + group scoping per `spec.md` §2 / §5.
- [ ] **T5.8** `[API]` Session CRUD (create, list, append messages or equivalent) and authz consistent with RBAC.
- [ ] **T5.9** `[FE]` Session list, resume, multi-turn UX; integrate SSE per turn within a persisted session id.
- [ ] **T5.10** `[FE]` Group-based **sharing** (two users / two browsers) when API supports it; manual RLS smoke (**005 checkpoint**).
- [ ] **T5.11** `[FE]` **US.5** cockpit evolution: wire to persisted usage when **`metrics`** (or equivalent) exists post-**M4b** Phase 5+.

**Process (Constitution XII / XIV)**: Before closing each vertical slice, run **`/speckit.checklist`** (or equivalent) and record layer gates in the PR when automation does not cover them.

---

## M5 “complete” vs product vision

Calling milestone **M5 done** against **`spec.md` §5–6** requires **M5b** (and, for **US.5** at spec depth, coordination with **M4b** metrics). **M5a** alone is a **deliberate, documented** subset (**§7**).

**Reference**: `specify/spec.md` §3, §5, §7; `specify/plan.md` §1–2; `specify/testing-checkpoints.md` (M5a vs M5b).
