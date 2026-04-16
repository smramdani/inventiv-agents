# MVP validation — Agent engine **without tools** and **without MCP**

This checklist is the **current** acceptance target for Milestone **M4 (MVP slice)** per roadmap update: prove identity, registry, LLM resolution, and **SSE completion** end-to-end before implementing MCP clients, tool execution, or run persistence (Phases 4–6 of `004_milestone_4.md`).

## How to run it (order)

1. Bring up infra and migrations (`make ready` or equivalent — works with **Docker Compose** or **host Postgres + `psql`** per README), then start the API (`make run` in a dedicated terminal).
2. Run **`make check`** on a machine with Docker so the full **`tests/*.rs`** suite passes (automated gate).
3. Run the **headless smoke** against your running API (manual gate): `make m4a-smoke` with **`M4A_LLM_API_KEY`** set to a **test** provider secret (see script header in `scripts/dev/m4a-mvp-smoke.sh`). This covers register → login → provider + key → agent → SSE in one shot.
4. Optionally repeat steps 1–3 of the **Manual** section by hand (`curl`) if you need to inspect headers or swap providers.
5. Fill **Sign-off** when satisfied.

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

**`make check`** automatically falls back to **`cargo test --lib`** when Docker/Compose is not on `PATH` (you still get `fmt` + `clippy` + unit tests). To **force** the no-Docker path without probing: `make check-local`.

For the **full** gate (including `tests/*.rs`), run **`make check`** on a machine where `docker compose version` succeeds (e.g. CI or Docker Desktop running).

- [ ] All unit tests pass (`src/**` `mod tests`, engine ports, LLM client wiremock).
- [ ] All integration tests pass (`tests/*.rs`), including:
  - [ ] `agents_api`, `agents_registry_rls`, `identity_rls`, `identity_http`
  - [ ] `llm_resolve_integration` (DB → resolver → mock LLM)
  - [ ] `sse_agent_stream_integration` (SSE `meta` / `delta` / `usage` / `done`, `X-Trace-ID`)

## Manual — headless MVP (no MCP, single user message)

Use a **test** API key and model on your provider (never commit secrets).

**Shortcut (recommended):** with the API running, `export M4A_LLM_API_KEY=…` then `make m4a-smoke` (same checks as steps 1–5 for a default OpenAI-compatible host; override `M4A_LLM_BASE_URL` / `M4A_LLM_MODEL` / `M4A_API_BASE` if needed).

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
