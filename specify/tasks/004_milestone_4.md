# Milestone 4: The Agentic Engine (LLM + SSE first; MCP / tools deferred)

**Artifact role**: **Implementation tasks** for the agentic engine. Each **`T###`** row declares **`Milestone`** (**M4a** kernel = Phases 1–3 (M5a-owned product **`US.x`**); **M4b** = Phases 4–6 with their own **M4b** **`US.x`**), **`Epic`** (`specify/spec.md` §2), and **exactly one** **`User story`** (`specify/spec.md` §7 **`US.1`–`US.15`**). **Task ids** (`T015` … `T032`) are **globally unique** and do **not** encode the milestone id. **Canonical matrix**: `specify/traceability.md`. Derived from **`specify/plan.md` §2.1** / **§2.4**.

**Prerequisites**: M3 complete (`specify/tasks/003_milestone_3.md`). **Plan**: `specify/plan.md` §2–3.

**Roadmap note (M5-first)**: **M5** (cockpit / front-end) is the **current** delivery priority. **Phases 4–6** of this file (**M4b** — MCP in loop, metrics persistence, orchestration) are **deferred until after M5**. **US.10** product wiring (MCP in loop, orchestration) ships with **M4b**, not before M5. Phase 4 code may exist as a **library**; **product integration** waits until **post-M5**.

**M4a focus**: Validate **Phases 1–3** — see **`specify/mvp-engine-validation.md`**.

## Layer definition of done (Constitution XIV)

Use as the **completion gate** for M4. Mark **N/A** only where noted.

### Vertical slice
- **Story linkage (M4a kernel for M5a)**: **US.7** (M5a) narrowed to **single-turn, no-tool** use: authenticated user invokes a configured agent (with LLM provider) and receives an **SSE** model response. **US.3**–**US.5** / **US.6** registry is supported by **M3** + resolver (**US.10** kernel tasks **T018**–**T019**). **US.4** skills and **US.5** toolbelt **in the reasoning loop** ship via **US.15** in **M4b** (with the user-facing **US.12** experience on top). **M5b** ships persisted **sessions** / sharing as **US.11** per `spec.md` §6; **US.8** (M5a) only covers the last-turn `usage` signal — persisted ground-truth metrics ship as **US.14** in **M4b**, and the owner dashboards as **US.13** in **M5b**.
- **Independent value**: A client can call the streaming completion API without M5 UI, without MCP, and without executing skills — demonstrable with `curl` + integration tests.
- **Artifacts**: `specify/spec.md`, `specify/plan.md`, `specify/mvp-engine-validation.md`, and this file stay aligned (XII); scope changes are updated in spec/plan first.

### Database `[DB]`
- **Each `[DB]` task in this file** satisfies: migrations reviewed; **RLS + FORCE RLS** on any new tenant tables; FK and uniqueness match execution and audit needs; token/cost persistence strategy documented (align with `metrics` / observability in `spec.md` §4).

### Domain & application `[Domain]`
- **Each `[Domain]` task in this file** satisfies: reasoning loop and tool orchestration live behind ports (traits); **`mod tests`** for state transitions, validation, and cost aggregation rules; no production `unwrap()`/`expect()` (VI).

### Repository / infrastructure `[DB]` boundary
- **Each repository-related task in this file** satisfies: persistence adapters use **`set_rls_context`** on every path that reads or writes tenant-scoped execution data; no business rules hidden in SQL beyond constraints.

### API `[API]`
- **Each `[API]` task in this file** satisfies: routes enforce **authn** (JWT) and **org-consistent authz** (role appropriate to “run agent”); SSE and JSON error shapes are documented; structured logging + **TraceID** on all paths (IX).

### Front-end `[FE]`
- **N/A for M4** (no cockpit UI; M5 owns chat UX and session sharing UI). Revisit XIV FE gates at **M5**.

### Cross-cutting
- **Security (M4a)**: LLM calls use org-scoped provider configuration only; secrets never logged; timeouts and payload limits on outbound HTTP. **MCP / tool sandboxing** applies from **M4b** onward.
- **Quality**: `cargo fmt`, `cargo clippy`, `cargo test` green; integration tests cover SSE happy path and auth/RLS as in existing suites. **MVP sign-off**: `specify/mvp-engine-validation.md`.
- **Constitution XV** (software **release** artifact — not a planning milestone): No environment-specific branching for deployment; feature flags only via config if needed.

## User story → tasks (this file)

Canonical split by milestone remains **`specify/traceability.md` §2**. **M4a** = Phases **1–3**; **M4b** = Phases **4–6**.

| **User story** | **`T###`** |
| :--- | :--- |
| **US.7** (M5a, kernel SSE chat) | **T016**, **T021**, **T022**, **T023** |
| **US.8** (M5a, last-turn `usage` data) | **T020** |
| **US.10** (M5a, streaming LLM kernel) | **T015**, **T017**, **T018**, **T019** |
| **US.14** (M4b, persisted runs / metrics) | **T027**, **T028**, **T029** |
| **US.15** (M4b, MCP + orchestrator kernel) | **T024**, **T025**, **T026**, **T030**, **T031**, **T032** |

---

## Phase 1 — Contracts & ports (blocking)

**Purpose**: Stable boundaries before HTTP and DB details.

- [x] **T015** `[Domain]` Define the **reasoning loop** model (states: reason → tool selection → execute → validate → respond, or equivalent) as pure domain types with errors in `backend/src/domain/` (new module e.g. `engine` or `reasoning`), with `mod tests` for invalid transitions.  
  **Milestone:** **M4a** · **Epic:** **E-ENG** · **User story:** **US.10** (orchestration contract).
- [x] **T016** `[Domain]` Define an **`LlmCompletionPort`** (trait): non-streaming and/or streaming chunk type, request context (model id, messages, max tokens), and structured errors—no `reqwest` in domain.  
  **Milestone:** **M4a** · **Epic:** **E-ENG** · **User story:** **US.10** (streaming port for **US.7**).
- [x] **T017** `[Domain]` Define an **`McpInvocationPort`** (trait): discover tools (if required by plan), invoke tool by name with JSON args, map timeouts and transport errors to domain errors—no raw JSON-RPC wire types leaked into handlers.  
  **Milestone:** **M4a** · **Epic:** **E-ENG** · **User story:** **US.10** (MCP invocation boundary).

**Checkpoint**: Traits compile; unit tests cover phase transitions and port call shapes (adapters in Phase 2+).

---

## Phase 2 — LLM adapter (OpenAI-compatible)

**Purpose**: Sovereign provider calls using org-stored provider URL + secret (M3).

- [x] **T018** `[Infra]` Implement **`OpenAiCompatibleClient`** (or similarly named) in `backend/src/infrastructure/llm/` using `reqwest` + streaming response handling; map HTTP and parse errors to domain errors.  
  **Milestone:** **M4a** · **Epic:** **E-ENG** · **User story:** **US.10** (LLM adapter).
- [x] **T019** `[Infra]` Wire **provider resolution** from `AgentsRepository` / provider id on the agent row; never log API keys (IX).  
  **Milestone:** **M4a** · **Epic:** **E-ENG** · **User story:** **US.10** (resolution uses **US.6** registry data).
- [x] **T020** `[Domain]` Token accounting model: capture input/output token counts (and optional cost fields) for each completion step for later persistence.  
  **Milestone:** **M4a** · **Epic:** **E-USAGE** · **User story:** **US.8** (groundwork for usage reporting).

**Checkpoint**: Wiremock-backed unit tests for JSON parsing and 429 mapping; DB helpers `get_agent_by_id` / `get_llm_provider_with_key` + `openai_compatible_client_for_agent`. Integration tests in `tests/llm_resolve_integration.rs` (DB seed → resolver → mock HTTP). See `specify/testing-checkpoints.md` for when to hit a real LLM.

---

## Phase 3 — SSE HTTP surface

**Purpose**: Real-time streaming to clients per **`spec.md` §8** (M4a) and **`004` Phases 1–3**.

- [x] **T021** `[API]` Add Axum route(s) for **streaming completion** (e.g. `POST /org/.../agents/{id}/complete/stream` or name per REST review)—document contract (headers, SSE event names, terminal event).  
  **Milestone:** **M4a** · **Epic:** **E-CHAT** · **User story:** **US.7**.
- [x] **T022** `[API]` Ensure **TraceID** propagation from request extension through LLM spans (IX). **MCP spans**: deferred to M4b.  
  **Milestone:** **M4a** · **Epic:** **E-CHAT** · **User story:** **US.7** (observability for agent conversations).
- [x] **T023** `[API]` Integration test: authenticated user receives SSE events for a minimal “no tools” completion (may use test double for LLM).  
  **Milestone:** **M4a** · **Epic:** **E-CHAT** · **User story:** **US.7**.

**Checkpoint**: `POST /org/agents/:agent_id/complete/stream` documented in README; `tests/sse_agent_stream_integration.rs`; structured logs include `trace_id` on stream open and LLM completion.

---

## Phase 4 — MCP client (minimal vertical slice) — **M4b (after M5)**

**Status**: HTTP JSON-RPC client **implemented** as a **library** (`backend/src/infrastructure/mcp/`). **Product wiring** (SSE + tools, orchestration) is **deferred until after M5** per `specify/plan.md`.

**Purpose**: JSON-RPC over HTTP(S) toward MCP servers registered as skills (M3).

- [x] **T024** `[Infra]` Implement MCP **tool list / invoke** client in `backend/src/infrastructure/mcp/` with strict timeouts and size limits; configuration from `Skill` rows (endpoint, metadata) via `McpHttpJsonRpcClient::new(skill_endpoint_url)`.  
  **Milestone:** **M4b** (library shipped pre-integration) · **Epic:** **E-ENG** · **User story:** **US.15**.
- [x] **T025** `[Domain]` Minimal mapping: `select_unique_tool_name` when the server exposes exactly one tool (orchestrator disambiguation later).  
  **Milestone:** **M4b** · **Epic:** **E-ENG** · **User story:** **US.15**.
- [x] **T026** `[Domain]` `validate_mcp_invoke_request` + `mod tests` for empty tool name and singleton selection.  
  **Milestone:** **M4b** · **Epic:** **E-ENG** · **User story:** **US.15**.

**Checkpoint**: Contract tests with **wiremock** stub JSON-RPC (`tools/list`, `tools/call`) in `backend/src/infrastructure/mcp/http_client.rs`.

---

## Phase 5 — Persistence for runs & metrics — **DEFERRED (M4b, after M5)**

**Purpose**: Align with `spec.md` §4 usage tracking and **US.8** groundwork. **After M5** (tokens already exposed in SSE `usage` for M4a).

- [ ] **T027** `[DB]` Add migration(s) for **execution or session run** tables (naming per implementation): `organization_id`, `agent_id`, `user_id`, `trace_id`, status, token fields, timestamps; **RLS** + **FORCE RLS**.  
  **Milestone:** **M4b** · **Epic:** **E-USAGE** · **User story:** **US.14** (ground-truth persistence for token metrics).
- [ ] **T028** `[Infra]` Repository for persisting runs and per-step metrics; **`set_rls_context`** on all queries; expose an **org-scoped read API** for **US.13** dashboards.  
  **Milestone:** **M4b** · **Epic:** **E-USAGE** · **User story:** **US.14**.
- [ ] **T029** `[Domain]` Link persisted run to the orchestrated stream so a completed turn leaves an auditable row (minimal fields acceptable for M4b initial slice).  
  **Milestone:** **M4b** · **Epic:** **E-USAGE** · **User story:** **US.14** (run linkage owned by the metrics ground-truth slice).

**Checkpoint**: Integration test proves org A cannot read org B’s runs (RLS).

---

## Phase 6 — End-to-end reasoning loop orchestration — **DEFERRED (M4b, after M5)**

**Purpose**: Connect loop + LLM + MCP + persistence in one service path (post–M5 cockpit).

- [ ] **T030** `[Domain]` / `[Infra]` Application service orchestrating the loop for one user message (happy path + tool failure + LLM failure).  
  **Milestone:** **M4b** · **Epic:** **E-ENG** · **User story:** **US.15** (orchestrator service).
- [ ] **T031** `[API]` Single entry route documented in `README.md` (minimal example for **tool-augmented** streaming agent chat).  
  **Milestone:** **M4b** · **Epic:** **E-ENG** · **User story:** **US.15** (entry route exposing the orchestrated kernel; the user-facing tool-augmented chat experience that ships on top is **US.12**).
- [ ] **T032** Cross-cutting: update **`CHANGELOG.md`** and **`README.md`** for new endpoints and env vars; run **`make check`** (or equivalent) before merge.  
  **Milestone:** **M4b** · **Epic:** **E-ENG** · **User story:** **US.15** (release documentation for kernel surfaces).

**Checkpoint**: XIV gates satisfied for M4b; update `specify/plan.md` when Phases 4–6 ship.

---

## Validation — **M4a MVP** (current)

Use **`specify/mvp-engine-validation.md`** as the authoritative checklist (automated + manual SSE without MCP).

- [ ] Automated: `make check` (or equivalent) — all tests green, including SSE + LLM resolver integration tests.
- [ ] Manual: `curl -N` SSE completion with a real test provider (optional but recommended once per environment).
- [ ] Sign-off line completed in `mvp-engine-validation.md`.

## Validation — **M4b** (after MVP; includes MCP + persistence)

- [ ] Integration test: **US.7**-style flow with **optional tool** path + persisted run/metrics where implemented.
- [ ] RLS test: cross-org denial on execution/run tables.
- [ ] Manual: MCP stub or sandbox (no secrets in repo).

---

## Dependencies (summary)

1. **M4a**: Phases **1 → 3**; validate with **`mvp-engine-validation.md`**.  
2. **M5**: Cockpit — **`005_milestone_5.md`** (**M5a** shipped, **M5b** sessions when scheduled).  
3. **M4b**: Phases **4–6** (MCP product integration, persistence, orchestration) **after M5** (per roadmap).  
4. Phase 2 before Phase 3 (streaming needs LLM adapter).

---

## Notes

- If scope is too large for one increment, split behind feature flags **via configuration** (XV), not compile-time `env == "prod"`.
- **Session sharing** and persisted **sessions** are **M5b** (`specify/spec.md` §6, `005_milestone_5.md`); **M5a** ships **single-turn** cockpit chat. **M4b** follows the **M5** milestone track per `specify/plan.md`.
- The domain trait **`McpInvocationPort`** (Phase 1) remains a boundary for M4b; **no MCP adapter** is required for M4a MVP sign-off.
