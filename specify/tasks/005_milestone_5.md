# Milestone 5: The Sovereign Cockpit (front-end + sessions)

**Artifact role**: **Implementation tasks** — each row below is one **verifiable** slice of work. Every task **must** declare **`Milestone`** (timeboxed target — same concept as **`specify/plan.md` §2** and story-map **`milestoneId`**; not Constitution **XV** software release), **`Epic`** (`specify/spec.md` §2 id), and **exactly one** **`User story`** (`specify/spec.md` §7 **`US.1`–`US.15`** — **§7.0** epic map + **§7.1–7.15** narratives). Each **`US.x`** is bound to a single milestone, so the **Epic** + **Milestone** on each task row must match the milestone the cited US owns. **Task ids** (`T033` … `T054`) are **globally unique** in this repo and do **not** encode the milestone id — **`Milestone`** on each row is the schedule truth. **Canonical matrix**: `specify/traceability.md`.

**Status**: **M5a shipped**; **M5b** and **M4b** remain. **Milestone backlog narrative**: `specify/plan.md` §2. **Prerequisites**: M4a engine (SSE + LLM) validated; M3 registry APIs available.

**Deferred until after M5 (product MCP path)**: **M4b** Phases 4–6 — see `specify/tasks/004_milestone_4.md`. HTTP MCP client ships as a **library** first; **live agent wiring** lands in **M4b**.

## User story → tasks (this file)

| **User story** | **`T###`** |
| :--- | :--- |
| **US.1** | **T033**, **T034**, **T046** |
| **US.2** | **T035**, **T036** |
| **US.3** | **T037**, **T038** |
| **US.4** | **T039**, **T040** |
| **US.5** | **T041**, **T042** |
| **US.7** (M5a) | **T043** |
| **US.8** (M5a) | **T044**, **T045** |
| **US.9** (M5b) | **T050**, **T051** |
| **US.11** (M5b) | **T047**, **T048**, **T049**, **T052** |
| **US.13** (M5b) | **T053**, **T054** |

## Delivery order (journey ↔ `plan.md` §2.2–2.3)

Execute / read in this order for **M5a**: **T033** → **T034–T036** → **T037–T042** → **T043** → **T044–T045** → **T046**. **M5b**: **T047** → **T048** → **T049** → **T050** → **T051** → **T052** → **T053** → **T054**.

## Purpose

Deliver the first **authenticated web client** so organizations can use the platform without `curl`: org context, LLM provider setup, user invite flows where applicable, **agent conversations** (SSE against M4a), and foundations for **usage / cost visibility**.

**Spec alignment (`specify/spec.md`)**: **§6** Sessions (full semantics **M5b**). **§4 / US.8**: last-turn **`usage`** in **M5a**; persisted **`metrics`** with **M4b** + **M5b** UI.

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

Constitution **XII**: each task below cites **`plan.md` §2.2**, **`spec.md` §2**, and **`spec.md` §7** (**§7.0** / **§7.1–7.10**).

### T033 — Monorepo + cockpit shell `[FE]`

| Field | Value |
| :--- | :--- |
| **Milestone** | **M5a** |
| **Epic** | **E-ONB** |
| **User story** | **US.1** |

- [x] **`backend/`** + **`frontend/`**; shared root tooling; Vite + React + TypeScript in `frontend/` per README.

### T034 — Register organization (first tenant) `[FE]`

| Field | Value |
| :--- | :--- |
| **Milestone** | **M5a** |
| **Epic** | **E-ONB** |
| **User story** | **US.1** |

- [x] **`POST /org/register`** and cockpit flow create org + Owner; data scoped to `organization_id`.

### T035 — Sign in + session JWT `[FE]`

| Field | Value |
| :--- | :--- |
| **Milestone** | **M5a** |
| **Epic** | **E-AUTH** |
| **User story** | **US.2** |

- [x] **`/auth/login`**, JWT in **sessionStorage** per **`spec.md` §8** (M5a); subsequent requests send `Authorization: Bearer …`.

### T036 — Resolve current user + role (`whoami`) `[FE]` / `[API]` consumer

| Field | Value |
| :--- | :--- |
| **Milestone** | **M5a** |
| **Epic** | **E-AUTH** |
| **User story** | **US.2** |

- [x] **`GET /auth/whoami`** returns **role**; cockpit gates Owner / Admin / User.

### T037 — List LLM providers (read) `[FE]`

| Field | Value |
| :--- | :--- |
| **Milestone** | **M5a** |
| **Epic** | **E-REG** |
| **User story** | **US.3** |

- [x] Cockpit lists org providers via M3 list API; org-scoped rows only.

### T038 — Create LLM provider (+ credentials) `[FE]`

| Field | Value |
| :--- | :--- |
| **Milestone** | **M5a** |
| **Epic** | **E-REG** |
| **User story** | **US.3** |

- [x] Create form posts to M3 create route; secrets not echoed in UI errors.

### T039 — List skills (read) `[FE]`

| Field | Value |
| :--- | :--- |
| **Milestone** | **M5a** |
| **Epic** | **E-REG** |
| **User story** | **US.4** |

- [x] Cockpit lists org skills for audit before agent configuration.

### T040 — Create skill (MCP or native) `[FE]`

| Field | Value |
| :--- | :--- |
| **Milestone** | **M5a** |
| **Epic** | **E-REG** |
| **User story** | **US.4** |

- [x] Create skill form persists via M3 API; appears in list.

### T041 — List agents (read) `[FE]`

| Field | Value |
| :--- | :--- |
| **Milestone** | **M5a** |
| **Epic** | **E-REG** |
| **User story** | **US.5** |

- [x] Members see agents allowed by API policy; list backs chat agent selection.

### T042 — Create agent (mission + provider) `[FE]`

| Field | Value |
| :--- | :--- |
| **Milestone** | **M5a** |
| **Epic** | **E-REG** |
| **User story** | **US.5** |

- [x] Create agent form persists; SSE completion can target new agent id.

### T043 — Single-turn SSE chat in cockpit `[FE]`

| Field | Value |
| :--- | :--- |
| **Milestone** | **M5a** |
| **Epic** | **E-CHAT** |
| **User story** | **US.7** |

- [x] **`POST /org/agents/:id/complete/stream`** via `fetch` + streaming parser; meta / delta / usage / done handled in UI.

### T044 — Capture last `usage` from SSE (client state) `[FE]`

| Field | Value |
| :--- | :--- |
| **Milestone** | **M5a** |
| **Epic** | **E-USAGE** |
| **User story** | **US.8** |

- [x] After `done`, client retains last non-null **usage** when API sent one.

### T045 — Owner “last usage” panel `[FE]`

| Field | Value |
| :--- | :--- |
| **Milestone** | **M5a** |
| **Epic** | **E-USAGE** |
| **User story** | **US.8** |

- [x] Panel renders captured **usage** fields; copy per **`spec.md` §4** (last-turn only).

### T046 — M5a cross-cutting docs + FE toolchain `[FE]` / docs

| Field | Value |
| :--- | :--- |
| **Milestone** | **M5a** |
| **Epic** | **E-ONB** |
| **User story** | **US.1** |

- [x] README, CHANGELOG, `specify/testing-checkpoints.md`; **`make fe-install` / `fe-dev` / `fe-build` / `fe-lint`**.

**Follow-up (security roadmap)**: httpOnly cookies / refresh tokens (**not** M5a scope).

**Checkpoint (M5a)**: Manual smoke: register → login → registry → chat with trace + usage; **`make check`** / **`make fe-lint`** green on touched code.

---

## M5b — planned (sessions + collaboration, `spec.md` §6)

### T047 — Session tables + RLS migrations `[DB]`

| Field | Value |
| :--- | :--- |
| **Milestone** | **M5b** |
| **Epic** | **E-CHAT** |
| **User story** | **US.11** |

- [ ] Migrations: `sessions` (+ related) with **RLS** / **FORCE RLS**; org + group scoping per **`spec.md` §3** / **§6**.

### T048 — Session HTTP API (CRUD + messages) `[API]`

| Field | Value |
| :--- | :--- |
| **Milestone** | **M5b** |
| **Epic** | **E-CHAT** |
| **User story** | **US.11** |

- [ ] Session create/list/append (or equivalent); authz matches RBAC + org scope.

### T049 — Session list, resume, multi-turn SSE `[FE]`

| Field | Value |
| :--- | :--- |
| **Milestone** | **M5b** |
| **Epic** | **E-CHAT** |
| **User story** | **US.11** |

- [ ] UI lists sessions, resume loads history, new prompt streams inside `session_id` context.

### T050 — Invite users (cockpit UI) `[FE]`

| Field | Value |
| :--- | :--- |
| **Milestone** | **M5b** |
| **Epic** | **E-GOV** |
| **User story** | **US.9** |

- [ ] Invite form + success/error paths; RBAC (Owner/Admin); errors omit secrets.

### T051 — Manage groups (cockpit UI) `[FE]`

| Field | Value |
| :--- | :--- |
| **Milestone** | **M5b** |
| **Epic** | **E-GOV** |
| **User story** | **US.9** |

- [ ] Create/list/edit (or subset per API); UI gating matches RBAC; aligns with session RLS policy inputs.

### T052 — Group-based session sharing `[FE]`

| Field | Value |
| :--- | :--- |
| **Milestone** | **M5b** |
| **Epic** | **E-CHAT** |
| **User story** | **US.11** |

- [ ] Two-browser smoke; RLS per **`specify/testing-checkpoints.md`**.

### T053 — Typed cockpit client for metrics read API `[FE]`

| Field | Value |
| :--- | :--- |
| **Milestone** | **M5b** |
| **Epic** | **E-USAGE** |
| **User story** | **US.13** |

- [ ] Client module + empty state until the **US.14** metrics read API (M4b Phase 5) exists.

### T054 — Owner usage / cost dashboard (persisted data) `[FE]`

| Field | Value |
| :--- | :--- |
| **Milestone** | **M5b** |
| **Epic** | **E-USAGE** |
| **User story** | **US.13** |

- [ ] Dashboards on live **metrics** contract post-**M4b** Phase 5+ (data persisted via **US.14**).

**Process (Constitution XII / XIV)**: Before closing each vertical slice, run **`/speckit.checklist`** (or equivalent) and record layer gates in the PR when automation does not cover them.

---

## M5 “complete” vs product vision

Calling milestone **M5 done** against **`spec.md` §6–7** requires **M5b** (and, for **US.8** at spec depth, coordination with **M4b** metrics). **M5a** is the **§8** cockpit slice shipped before **M5b** sessions and full **US.8** metrics.

**Reference**: `specify/spec.md` §4, §6, §8; `specify/plan.md` §1–2; `specify/testing-checkpoints.md` (M5a vs M5b).
