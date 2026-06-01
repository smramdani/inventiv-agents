# Milestone 3: Registry & Entities (Providers, Skills, Agents)

**Artifact role**: **Implementation tasks** for the **headless registry**. Each **`T###`** row below declares **`Milestone`:** **M3** (shipped), **`Epic`** (`specify/spec.md` §2), and **exactly one** **`User story`** (`specify/spec.md` §7 **`US.1`–`US.15`**). **Task ids** (`T006` … `T014`) are **globally unique** in this repo and do **not** encode the milestone id. **Canonical matrix**: `specify/traceability.md`. The **US.3**–**US.6** product user stories themselves are **M5a** (cockpit + API surface); **M3** ships the data layer those stories depend on.

## Layer definition of done (Constitution XIV)

Use this as the **completion gate** for M3 before calling the milestone done. Mark **N/A** only where noted.

### Vertical slice
- **Story linkage**: Work traces to **US.3** (provider catalog), **US.4** (skill catalog), **US.5** (agent catalog), and **US.6** (tenant-isolated registry platform) in `specify/spec.md` §7.
- **Independent value**: Admin/Owner can persist providers, skills, and agents with correct org scope via repository + API path; demonstrable without chat UI (M4/M5).
- **Artifacts**: When behavior ships, `specify/spec.md` / `specify/plan.md` / this file stay aligned (XII); any scope change is updated there first.

### Database `[DB]`
- **Tasks T006–T009** satisfy: migration reviewed, upgrade path clear, **RLS + FORCE RLS** on all new tables, FK/uniqueness match domain rules.

### Backend — domain `[Domain]`
- **Tasks T010–T012** satisfy: validation and types in domain modules; **`mod tests`** for URL/mission/persona/skill rules; no production `unwrap()`/`expect()` (VI).

### Repository `[DB]` + `[Domain]` boundary
- **Task T013** satisfies: `AgentsRepository` maps rows ↔ domain without leaking business rules into SQL beyond constraints; **`set_rls_context`** on every mutating and read path that must respect tenant.

### API `[API]`
- **Task T014** satisfies: handlers enforce **authn + Admin/Owner** (and org scope consistent with RLS); validation errors vs auth errors are distinct; structured logging + **TraceID** on success and failure (IX).

### Front-end `[FE]`
- **N/A for M3** (no cockpit UI in this milestone per `specify/plan.md` M3 scope). Revisit XIV FE gates at **M5**.

### Cross-cutting
- **Validation**: The **Validation** checklist below proves **US.5**-style flows and **RLS isolation**; `cargo fmt`, `cargo clippy`, and `cargo test` are green for touched crates.
- **Security**: No secrets in logs; API keys stored per plan (encrypted at rest as specified in tasks/schema).

## User story → tasks (this milestone)

| **User story** | **`T###`** |
| :--- | :--- |
| **US.3** | **T006**, **T010** |
| **US.4** | **T007**, **T011** |
| **US.5** | **T008**, **T012** |
| **US.6** | **T009**, **T013**, **T014** |

## Tasks

### T006 — Migration: `llm_providers` + keys `[DB]`

**Milestone:** **M3** · **Epic:** **E-REG** · **User story:** **US.3**

- [x] `llm_providers` table (ID, URL, encrypted API Key) in `003_agents_registry.sql` (or equivalent migration chain).

### T007 — Migration: `skills` `[DB]`

**Milestone:** **M3** · **Epic:** **E-REG** · **User story:** **US.4**

- [x] `skills` table (type MCP/Native, endpoint/code, configuration).

### T008 — Migration: `agents` + `agent_skills` `[DB]`

**Milestone:** **M3** · **Epic:** **E-REG** · **User story:** **US.5**

- [x] `agents` table (Mission, Persona, LLM choice) and **`agent_skills`** junction.

### T009 — RLS, indexes, grants `[DB]`

**Milestone:** **M3** · **Epic:** **E-REG** · **User story:** **US.6**

- [x] **RLS** + **FORCE RLS** on all new tables; follow-up `004_agents_registry_indexes_and_grants.sql` (indexes on `organization_id`, `GRANT USAGE ON TYPE skill_type`).

### T010 — Domain: provider model `[Domain]`

**Milestone:** **M3** · **Epic:** **E-REG** · **User story:** **US.3**

- [x] `src/domain/agents/provider.rs` + **`mod tests`** (URL / key validation rules).

### T011 — Domain: skill model `[Domain]`

**Milestone:** **M3** · **Epic:** **E-REG** · **User story:** **US.4**

- [x] `src/domain/agents/skill.rs` + **`mod tests`**.

### T012 — Domain: agent model `[Domain]`

**Milestone:** **M3** · **Epic:** **E-REG** · **User story:** **US.5**

- [x] `src/domain/agents/agent.rs` + **`mod tests`** (mission, persona, tool references).

### T013 — Repository adapters `[DB]` + `[Domain]`

**Milestone:** **M3** · **Epic:** **E-REG** · **User story:** **US.6**

- [x] `AgentsRepository` in `src/infrastructure/database/agents.rs`; **`set_rls_context`** on all operations.

### T014 — API: registry management handlers `[API]`

**Milestone:** **M3** · **Epic:** **E-REG** · **User story:** **US.6**

- [x] Handlers for registering **providers**, **skills**, and **agents**; Admin/Owner authn/authz; **TraceID** logging (IX).
- [x] Responses and persistence remain consistent with **`agent_skills`** and RLS as defined in migrations for this milestone.

## Validation
- [x] Integration tests verify that an Admin can create an Agent with multiple Skills.
- [x] RLS check: Org A cannot see Org B's Agents.
