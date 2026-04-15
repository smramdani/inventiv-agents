# MVP validation — Agent engine **without tools** and **without MCP**

This checklist is the **current** acceptance target for Milestone **M4 (MVP slice)** per roadmap update: prove identity, registry, LLM resolution, and **SSE completion** end-to-end before implementing MCP clients, tool execution, or run persistence (Phases 4–6 of `004_milestone_4.md`).

## Preconditions

- [ ] Docker Postgres + Redis up (`make ready` or equivalent).
- [ ] Migrations applied through **`005`** (includes `lookup_user_for_login` for login under RLS).
- [ ] `.env` with valid `DATABASE_URL`, `JWT_SECRET`.

## Automated (CI / local)

Run from repo root:

```bash
make check
# or: cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test
```

If **Docker is not available** (e.g. not on `PATH`), use:

```bash
make check-local
```

That runs `fmt`, `clippy`, and **`cargo test --lib`** only (library unit tests — no `tests/*.rs` integration suite). Run **`make check`** in CI or on a machine with Docker for the full gate including integration tests.

- [ ] All unit tests pass (`src/**` `mod tests`, engine ports, LLM client wiremock).
- [ ] All integration tests pass (`tests/*.rs`), including:
  - [ ] `agents_api`, `agents_registry_rls`, `identity_rls`, `identity_http`
  - [ ] `llm_resolve_integration` (DB → resolver → mock LLM)
  - [ ] `sse_agent_stream_integration` (SSE `meta` / `delta` / `usage` / `done`, `X-Trace-ID`)

## Manual — headless MVP (no MCP, single user message)

Use a **test** API key and model on your provider (never commit secrets).

1. [ ] **Register + login** (or seed org + admin in DB) — obtain JWT.
2. [ ] **Create LLM provider** with `base_url` + `api_key` pointing to a compatible endpoint (or tunnel to wiremock).
3. [ ] **Create agent** with `llm_provider_id` set.
4. [ ] **SSE completion**: `curl -N` to `POST /org/agents/<id>/complete/stream` with JSON `{"message":"...","model":"<id>"}` and `Authorization: Bearer …`.
5. [ ] Confirm response: `Content-Type: text/event-stream`, response header `X-Trace-ID`, body contains `event: meta`, `event: delta`, `event: usage`, `event: done`.
6. [ ] Confirm logs (optional): structured lines include `trace_id` for the stream.

## Explicitly **out of scope** for this MVP gate

- MCP tool list / invoke (`T4.10`–`T4.12`) — **deferred** until after this checklist is signed off.
- Reasoning-loop orchestration with tool branches (`T4.16`+) — **deferred**.
- Persistence of runs / metrics tables (`T4.13`–`T4.15`) — **deferred** (token counts already returned in SSE `usage` event).

## Sign-off

- **Owner / Tech lead**: name, date — MVP engine (no tools, no MCP) validated: **yes / no**.

When **yes**, proceed to roadmap **M4b** (MCP + tools) or **M5** planning per `specify/plan.md`.
