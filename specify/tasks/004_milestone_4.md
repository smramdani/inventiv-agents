# Milestone 4: The Agentic Engine (LLM streaming, MCP client, reasoning loop)

**Prerequisites**: M3 complete (`specify/tasks/003_milestone_3.md`). **Plan**: `specify/plan.md` §2–3.

## Layer definition of done (Constitution XIV)

Use as the **completion gate** for M4. Mark **N/A** only where noted.

### Vertical slice
- **Story linkage**: Work traces to **US.4** (user starts a session with an agent) for the **headless** path: authenticated user invokes a configured agent and receives a streamed model response; **US.1–US.3** supply providers, skills, and agents already persisted in M3.
- **Independent value**: A client can call a documented HTTP API (including SSE) without the M5 cockpit UI; behavior is testable with `curl` or integration tests.
- **Artifacts**: `specify/spec.md`, `specify/plan.md`, and this file stay aligned (XII); scope changes are updated in spec/plan first.

### Database `[DB]`
- **T4.x** satisfies: migrations reviewed; **RLS + FORCE RLS** on any new tenant tables; FK and uniqueness match execution and audit needs; token/cost persistence strategy documented (align with `metrics` / observability in spec §3).

### Domain & application `[Domain]`
- **T4.x** satisfies: reasoning loop and tool orchestration live behind ports (traits); **`mod tests`** for state transitions, validation, and cost aggregation rules; no production `unwrap()`/`expect()` (VI).

### Repository / infrastructure `[DB]` boundary
- **T4.x** satisfies: persistence adapters use **`set_rls_context`** on every path that reads or writes tenant-scoped execution data; no business rules hidden in SQL beyond constraints.

### API `[API]`
- **T4.x** satisfies: routes enforce **authn** (JWT) and **org-consistent authz** (role appropriate to “run agent”); SSE and JSON error shapes are documented; structured logging + **TraceID** on all paths (IX).

### Front-end `[FE]`
- **N/A for M4** (no cockpit UI; M5 owns chat UX and session sharing UI). Revisit XIV FE gates at **M5**.

### Cross-cutting
- **Security**: MCP and LLM calls use org-scoped configuration only; secrets never logged; timeouts and payload limits on outbound HTTP.
- **Quality**: `cargo fmt`, `cargo clippy`, `cargo test` green; integration tests cover SSE happy path and at least one auth/RLS failure.
- **Release (XV)**: No environment-specific branching for deployment; feature flags only via config if needed.

---

## Phase 1 — Contracts & ports (blocking)

**Purpose**: Stable boundaries before HTTP and DB details.

- [x] **T4.1** `[Domain]` Define the **reasoning loop** model (states: reason → tool selection → execute → validate → respond, or equivalent) as pure domain types with errors in `src/domain/` (new module e.g. `engine` or `reasoning`), with `mod tests` for invalid transitions.
- [x] **T4.2** `[Domain]` Define an **`LlmCompletionPort`** (trait): non-streaming and/or streaming chunk type, request context (model id, messages, max tokens), and structured errors—no `reqwest` in domain.
- [x] **T4.3** `[Domain]` Define an **`McpInvocationPort`** (trait): discover tools (if required by plan), invoke tool by name with JSON args, map timeouts and transport errors to domain errors—no raw JSON-RPC wire types leaked into handlers.

**Checkpoint**: Traits compile; unit tests cover phase transitions and port call shapes (adapters in Phase 2+).

---

## Phase 2 — LLM adapter (OpenAI-compatible)

**Purpose**: Sovereign provider calls using org-stored provider URL + secret (M3).

- [x] **T4.4** `[Infra]` Implement **`OpenAiCompatibleClient`** (or similarly named) in `src/infrastructure/llm/` using `reqwest` + streaming response handling; map HTTP and parse errors to domain errors.
- [x] **T4.5** `[Infra]` Wire **provider resolution** from `AgentsRepository` / provider id on the agent row; never log API keys (IX).
- [x] **T4.6** `[Domain]` **[US.1]** Token accounting model: capture input/output token counts (and optional cost fields) for each completion step for later persistence.

**Checkpoint**: Wiremock-backed unit tests for JSON parsing and 429 mapping; DB helpers `get_agent_by_id` / `get_llm_provider_with_key` + `openai_compatible_client_for_agent`. See `specify/testing-checkpoints.md` for when to hit a real LLM.

---

## Phase 3 — SSE HTTP surface

**Purpose**: Real-time streaming to clients per plan §3.

- [ ] **T4.7** `[API]` Add Axum route(s) for **streaming completion** (e.g. `POST /org/.../agents/{id}/complete/stream` or name per REST review)—document contract (headers, SSE event names, terminal event).
- [ ] **T4.8** `[API]` Ensure **TraceID** propagation from request extension through LLM and MCP spans (IX).
- [ ] **T4.9** `[API]` Integration test: authenticated user receives SSE events for a minimal “no tools” completion (may use test double for LLM).

**Checkpoint**: `curl`/test client can consume SSE end-to-end in tests.

---

## Phase 4 — MCP client (minimal vertical slice)

**Purpose**: JSON-RPC over HTTP(S) toward MCP servers registered as skills (M3).

- [ ] **T4.10** `[Infra]` Implement MCP **tool list / invoke** client in `src/infrastructure/mcp/` with strict timeouts and size limits; configuration from `Skill` rows (endpoint, metadata).
- [ ] **T4.11** `[Domain]` **[US.2]** Map MCP tool definitions to the reasoning loop’s tool-selection step (subset acceptable for first slice: single tool invocation on demand).
- [ ] **T4.12** `[Domain]` `mod tests` for MCP argument validation and error mapping.

**Checkpoint**: At least one integration or contract-style test against a stub MCP server or hyper test server.

---

## Phase 5 — Persistence for runs & metrics

**Purpose**: Align with spec §3 usage tracking and US.5 groundwork.

- [ ] **T4.13** `[DB]` Add migration(s) for **execution or session run** tables (naming per implementation): `organization_id`, `agent_id`, `user_id`, `trace_id`, status, token fields, timestamps; **RLS** + **FORCE RLS**.
- [ ] **T4.14** `[Infra]` Repository for persisting runs and per-step metrics; **`set_rls_context`** on all queries.
- [ ] **T4.15** `[Domain]` **[US.4]** Link persisted run to API flow so a completed stream leaves an auditable row (minimal fields acceptable for M4).

**Checkpoint**: Integration test proves org A cannot read org B’s runs (RLS).

---

## Phase 6 — End-to-end reasoning loop orchestration

**Purpose**: Connect loop + LLM + optional MCP + persistence in one service path.

- [ ] **T4.16** `[Domain]` / `[Infra]` Application service orchestrating the loop for one user message (happy path + tool failure + LLM failure).
- [ ] **T4.17** `[API]` **[US.4]** Single entry route documented in `README.md` (minimal example for streaming agent chat).
- [ ] **T4.18** Cross-cutting: update **`CHANGELOG.md`** and **`README.md`** for new endpoints and env vars; run **`make check`** (or equivalent) before merge.

**Checkpoint**: XIV gates above satisfied; M4 marked done in `specify/plan.md` when this phase is complete.

---

## Validation (milestone sign-off)

- [ ] Integration test: **US.4-style** flow—create org/user context, use M3 APIs to register provider + agent, then call M4 streaming endpoint and assert SSE + persisted trace/metrics.
- [ ] RLS test: cross-org denial on execution/run tables.
- [ ] Manual or scripted note: document how to run against a real MCP server in dev (without committing secrets).

---

## Dependencies (summary)

1. Phase 1 before 2–6.  
2. Phase 2 before 3 (streaming needs LLM adapter) and before 6.  
3. Phase 4 can overlap Phase 2 after T4.2–T4.3 are stable.  
4. Phase 5 before final persistence assertions in Phase 6.  
5. Phase 6 last.

---

## Notes

- If scope is too large for one increment, split behind feature flags **via configuration** (XV), not compile-time `env == "prod"`.
- Full **session sharing** and cockpit UX remain **M5** per `specify/plan.md`.
