# When to test the application (by milestone)

This note complements **Constitution VIII** (tests) and **XIV** (layer gates). It answers: *when should we run the real stack (Docker, API, optional external LLM) before going further?*

## Every milestone (always)

- After each **task group / PR**: `make check` or at least `cargo test` (unit + integration where Docker is available).
- Before calling a milestone **done**: satisfy the **Validation** section in that milestone’s task file (e.g. `specify/tasks/003_milestone_3.md`, `004_milestone_4.md`).

## M1–M3 (foundation + registry) — already shipped

| Checkpoint | What to run manually |
|------------|----------------------|
| After DB migrations | `make doctor` / `make ready`, then `make migrate` (or reset if schema conflict). |
| After auth / org APIs | `curl` flows from `README.md` (register, login). |
| After registry (M3) | Integration tests + optional `curl` to `/org/providers`, `/org/agents` with a JWT. |

## M4 (agentic engine) — recommended manual gates

1. **After Phase 1 (domain ports)** — Mostly **automated** (`cargo test`); no full-app requirement yet.
2. **After Phase 2 (LLM HTTP client + DB resolution)** — **Manual or staging**: `make ready`, create provider with a **test** API key and an agent with `llm_provider_id`, then call a small **Rust test binary** or temporary route that uses `openai_compatible_client_for_agent` + `complete` against a **cheap/safe** model (or keep relying on **wiremock** in CI only). Goal: prove real TLS + URL + key from DB.
3. **After Phase 3 (SSE route)** — **Manual**: `curl -N` (or similar) against the streaming endpoint; confirm chunks and terminal event.
4. **After Phase 4 (MCP)** — **Manual**: run against a **local stub MCP** or known public sandbox; confirm timeout and error paths.
5. **After Phase 5–6 (persistence + orchestration)** — **Manual + DB**: verify rows in execution/metrics tables and RLS with two orgs.

**Rule of thumb**: introduce a **real provider / real MCP** only at the first point where the **vertical slice** would otherwise be untrusted; until then, **integration tests + wiremock** reduce cost and flakiness.

## M5 (cockpit) — recommended manual gates

- After first **authenticated UI** path: smoke in browser (login, empty states).
- After **session sharing**: two browsers / two users, RLS checks.
- Before release: **accessibility**, **audit log** export, and **cost dashboard** spot-check against known test data.

## Cross-cutting

- **`make full`** (or CI): before tagging a release or promoting an artifact (**Constitution XV**).
- **Never** commit real production keys; use `.env` and secret stores for manual LLM tests.
