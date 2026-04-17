# Milestone 4: The Agentic Engine (LLM + SSE first; MCP / tools deferred)

**Prerequisites**: M3 complete (`specify/tasks/003_milestone_3.md`). **Plan**: `specify/plan.md` §2–3 (roadmap **M4a** = MVP without tools/MCP; **M4b** = MCP + persistence + full loop).

**MVP focus (now)**: Validate **Phases 1–3** only — see **`specify/mvp-engine-validation.md`**. Do **not** implement Phase 4 (MCP) until that checklist is signed off.

## Layer definition of done (Constitution XIV)

Use as the **completion gate** for M4. Mark **N/A** only where noted.

### Vertical slice
- **Story linkage (M4a MVP)**: **US.4** narrowed to **single-turn, no-tool** use: authenticated user invokes a configured agent (with LLM provider) and receives an **SSE** model response. **US.1–US.3** supply provider + agent registration. **US.2** (MCP skills in the loop) is **out of scope** until M4b.
- **Independent value**: A client can call the streaming completion API without M5 UI, without MCP, and without executing skills — demonstrable with `curl` + integration tests.
- **Artifacts**: `specify/spec.md`, `specify/plan.md`, `specify/mvp-engine-validation.md`, and this file stay aligned (XII); scope changes are updated in spec/plan first.

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
- **Security (M4a)**: LLM calls use org-scoped provider configuration only; secrets never logged; timeouts and payload limits on outbound HTTP. **MCP / tool sandboxing** applies from **M4b** onward.
- **Quality**: `cargo fmt`, `cargo clippy`, `cargo test` green; integration tests cover SSE happy path and auth/RLS as in existing suites. **MVP sign-off**: `specify/mvp-engine-validation.md`.
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

**Checkpoint**: Wiremock-backed unit tests for JSON parsing and 429 mapping; DB helpers `get_agent_by_id` / `get_llm_provider_with_key` + `openai_compatible_client_for_agent`. Integration tests in `tests/llm_resolve_integration.rs` (DB seed → resolver → mock HTTP). See `specify/testing-checkpoints.md` for when to hit a real LLM.

---

## Phase 3 — SSE HTTP surface

**Purpose**: Real-time streaming to clients per plan §3.

- [x] **T4.7** `[API]` Add Axum route(s) for **streaming completion** (e.g. `POST /org/.../agents/{id}/complete/stream` or name per REST review)—document contract (headers, SSE event names, terminal event).
- [x] **T4.8** `[API]` Ensure **TraceID** propagation from request extension through LLM spans (IX). **MCP spans**: deferred to M4b.
- [x] **T4.9** `[API]` Integration test: authenticated user receives SSE events for a minimal “no tools” completion (may use test double for LLM).

**Checkpoint**: `POST /org/agents/:agent_id/complete/stream` documented in README; `tests/sse_agent_stream_integration.rs`; structured logs include `trace_id` on stream open and LLM completion.

---

## Phase 4 — MCP client (minimal vertical slice) — **M4b (in progress)**

**Status**: First HTTP JSON-RPC slice shipped; orchestration in Phase 6 still wires this into the live API path.

**Purpose**: JSON-RPC over HTTP(S) toward MCP servers registered as skills (M3).

- [x] **T4.10** `[Infra]` Implement MCP **tool list / invoke** client in `src/infrastructure/mcp/` with strict timeouts and size limits; configuration from `Skill` rows (endpoint, metadata) via `McpHttpJsonRpcClient::new(skill_endpoint_url)`.
- [x] **T4.11** `[Domain]` **[US.2]** Minimal mapping: `select_unique_tool_name` when the server exposes exactly one tool (orchestrator disambiguation later).
- [x] **T4.12** `[Domain]` `validate_mcp_invoke_request` + `mod tests` for empty tool name and singleton selection.

**Checkpoint**: Contract tests with **wiremock** stub JSON-RPC (`tools/list`, `tools/call`) in `src/infrastructure/mcp/http_client.rs`.

---

## Phase 5 — Persistence for runs & metrics — **DEFERRED (M4b)**

**Purpose**: Align with spec §3 usage tracking and US.5 groundwork. **Out of MVP** (tokens already exposed in SSE `usage` for M4a).

- [ ] **T4.13** `[DB]` Add migration(s) for **execution or session run** tables (naming per implementation): `organization_id`, `agent_id`, `user_id`, `trace_id`, status, token fields, timestamps; **RLS** + **FORCE RLS**.
- [ ] **T4.14** `[Infra]` Repository for persisting runs and per-step metrics; **`set_rls_context`** on all queries.
- [ ] **T4.15** `[Domain]` **[US.4]** Link persisted run to API flow so a completed stream leaves an auditable row (minimal fields acceptable for M4).

**Checkpoint**: Integration test proves org A cannot read org B’s runs (RLS).

---

## Phase 6 — End-to-end reasoning loop orchestration — **DEFERRED (M4b)**

**Purpose**: Connect loop + LLM + MCP + persistence in one service path.

- [ ] **T4.16** `[Domain]` / `[Infra]` Application service orchestrating the loop for one user message (happy path + tool failure + LLM failure).
- [ ] **T4.17** `[API]` **[US.4]** Single entry route documented in `README.md` (minimal example for streaming agent chat).
- [ ] **T4.18** Cross-cutting: update **`CHANGELOG.md`** and **`README.md`** for new endpoints and env vars; run **`make check`** (or equivalent) before merge.

**Checkpoint**: XIV gates satisfied for M4b; update `specify/plan.md` when Phases 4–6 ship.

---

## Validation — **M4a MVP** (current)

Use **`specify/mvp-engine-validation.md`** as the authoritative checklist (automated + manual SSE without MCP).

- [ ] Automated: `make check` (or equivalent) — all tests green, including SSE + LLM resolver integration tests.
- [ ] Manual: `curl -N` SSE completion with a real test provider (optional but recommended once per environment).
- [ ] Sign-off line completed in `mvp-engine-validation.md`.

## Validation — **M4b** (after MVP; includes MCP + persistence)

- [ ] Integration test: **US.4-style** flow with **optional tool** path + persisted run/metrics where implemented.
- [ ] RLS test: cross-org denial on execution/run tables.
- [ ] Manual: MCP stub or sandbox (no secrets in repo).

---

## Dependencies (summary)

1. **M4a**: Phases **1 → 2 → 3**; validate with **`mvp-engine-validation.md`** (manual SSE + sign-off still recommended for production readiness).  
2. **M4b**: Phase **4** (MCP HTTP client) is underway; **5** (persistence) before persistence-heavy assertions in **6** (orchestration).  
3. Phase 2 before Phase 3 (streaming needs LLM adapter).

---

## Notes

- If scope is too large for one increment, split behind feature flags **via configuration** (XV), not compile-time `env == "prod"`.
- Full **session sharing** and cockpit UX remain **M5** per `specify/plan.md`.
- The domain trait **`McpInvocationPort`** (Phase 1) remains a boundary for M4b; **no MCP adapter** is required for M4a MVP sign-off.
