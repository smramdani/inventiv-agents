# Milestone 3: Registry & Entities (Providers, Skills, Agents)

## Layer definition of done (Constitution XIV)

Use this as the **completion gate** for M3 before calling the milestone done. Mark **N/A** only where noted.

### Vertical slice
- **Story linkage**: Work traces to **US.1** (provider registration), **US.2** (MCP skill), and **US.3** (agent with mission + skills) in `specify/spec.md` §6.
- **Independent value**: Admin/Owner can persist providers, skills, and agents with correct org scope via repository + API path; demonstrable without chat UI (M4/M5).
- **Artifacts**: When behavior ships, `specify/spec.md` / `specify/plan.md` / this file stay aligned (XII); any scope change is updated there first.

### Database `[DB]`
- **T3.1** satisfies: migration reviewed, upgrade path clear, **RLS + FORCE RLS** on all new tables, FK/uniqueness match domain rules.

### Backend — domain `[Domain]`
- **T3.2** satisfies: validation and types in domain modules; **`mod tests`** for URL/mission/persona/skill rules; no production `unwrap()`/`expect()` (VI).

### Repository `[DB]` + `[Domain]` boundary
- **T3.3** satisfies: `AgentsRepository` maps rows ↔ domain without leaking business rules into SQL beyond constraints; **`set_rls_context`** on every mutating and read path that must respect tenant.

### API `[API]`
- **T3.4** satisfies: handlers enforce **authn + Admin/Owner** (and org scope consistent with RLS); validation errors vs auth errors are distinct; structured logging + **TraceID** on success and failure (IX).

### Front-end `[FE]`
- **N/A for M3** (no cockpit UI in this milestone per `specify/plan.md` M3 scope). Revisit XIV FE gates at **M5**.

### Cross-cutting
- **Validation**: The **Validation** checklist below proves US.3-style flows and **RLS isolation**; `cargo fmt`, `cargo clippy`, and `cargo test` are green for touched crates.
- **Security**: No secrets in logs; API keys stored per plan (encrypted at rest as specified in tasks/schema).

## Tasks

### T3.1: Database Schema Migration
- [x] Create `003_agents_registry.sql` migration.
- [x] Add `llm_providers` table (ID, URL, encrypted API Key).
- [x] Add `skills` table (ID, type: MCP/Native, endpoint/code, configuration).
- [x] Add `agents` table (ID, Mission, Persona, LLM choice).
- [x] Add `agent_skills` junction table.
- [x] Enable **RLS** and **FORCE RLS** on all new tables.
- [x] Follow-up migration `004_agents_registry_indexes_and_grants.sql` (indexes on `organization_id`, `GRANT USAGE ON TYPE skill_type`).

### T3.2: Domain Models Implementation
- [x] Create `src/domain/agents/provider.rs`.
- [x] Create `src/domain/agents/skill.rs`.
- [x] Create `src/domain/agents/agent.rs`.
- [x] Write TDD unit tests for each model (validation of URLs, missions, etc.).

### T3.3: Repository Adapters
- [x] Implement `AgentsRepository` in `src/infrastructure/database/agents.rs`.
- [x] Ensure `set_rls_context` is used for all operations.

### T3.4: Management API Handlers
- [x] Implement handlers for registering Providers, Skills, and Agents.
- [x] Protected by Admin/Owner role.

## Validation
- [x] Integration tests verify that an Admin can create an Agent with multiple Skills.
- [x] RLS check: Org A cannot see Org B's Agents.
