# Milestone 1: Technical Foundation & LLM Connectivity

**Artifact role**: **Historical / scaffold** checklist from early repo bootstrap. Treat as **superseded** by the shipped stack under **`backend/`** and later milestones (**M3**–**M5**). If revived for a greenfield crate, each task should still declare **`Milestone`**, **`Epic`**, **`User story`** like later milestone files. **Task ids** **`T001`–`T005`** are **globally unique** (see `specify/plan.md` §2.0) and do not encode a milestone number.

**Suggested trace (if reusing this list):** **Milestone:** pre-**M3** scaffold · **Epic:** **E-ENG** · **User story:** **US.10** (execution kernel scaffold).

## Initialization tasks (Cargo & Rust)

### T001 — `Cargo.toml` configuration

**Milestone:** (scaffold) · **Epic:** **E-ENG** · **User story:** **US.10**

- [ ] Add dependencies: `tokio`, `axum`, `serde`, `serde_json`, `reqwest`, `anyhow`, `thiserror`, `tower-http`.
- [ ] Verify the project compiles with `cargo check`.

### T002 — Base architecture (folders & modules)

**Milestone:** (scaffold) · **Epic:** **E-ENG** · **User story:** **US.10**

- [ ] Create base folders: `src/api`, `src/core`, `src/infrastructure`.
- [ ] Declare modules in `src/main.rs`.

### T003 — `Agent` trait definition

**Milestone:** (scaffold) · **Epic:** **E-ENG** · **User story:** **US.10** (early agent abstraction; superseded by domain layout in **M3** / **M4**)

- [ ] Create the `Agent` trait in `src/core/agent.rs`.
- [ ] The trait must include an async method to process a message.

### T004 — “Hello World” API server (Axum)

**Milestone:** (scaffold) · **Epic:** **E-ENG** · **User story:** **US.10** (minimal HTTP surface enabler)

- [ ] Create a minimal Axum server with a healthcheck endpoint.
- [ ] Verify it runs locally.

### T005 — Base LLM client (provider)

**Milestone:** (scaffold) · **Epic:** **E-ENG** · **User story:** **US.10** (outbound LLM call shape)

- [ ] Create a generic HTTP client to call an API (e.g. OpenAI or Ollama).
- [ ] Map requests and responses to Rust structs.

## Milestone completion checklist

- [ ] The server starts without errors.
- [ ] An HTTP request can be sent and the agent responds with a simple message.
- [ ] Basic tests pass.
