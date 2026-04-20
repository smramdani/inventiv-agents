# When to test the application (by milestone)

This note complements **Constitution VIII** (tests) and **XIV** (layer gates). It answers: *when should we run the real stack (Docker, API, optional external LLM) before going further?*

## Every milestone (always)

- After each **task group / PR**: `make check` or at least `cargo test` (unit + integration where Docker is available). After **`frontend/`** changes: also **`make fe-lint`** (and **`make fe-build`** before release).
- After changes to **`scripts/dev/lib.sh`** or Docker-related workflows: **`make verify-bootstrap`** (stubbed `docker` / stripped `PATH`; no daemon required).
- Before calling a milestone **done**: satisfy the **Validation** section in that milestone’s task file (e.g. `specify/tasks/003_milestone_3.md`, `004_milestone_4.md`).

## M1–M3 (foundation + registry) — already shipped

| Checkpoint | What to run manually |
|------------|----------------------|
| After DB migrations | `make doctor` / `make ready`, then `make migrate` (or reset if schema conflict). |
| After auth / org APIs | `curl` flows from `README.md` (register, login). |
| After registry (M3) | Integration tests + optional `curl` to `/org/providers`, `/org/agents` with a JWT. |

## M4a — MVP engine (**no tools, no MCP**)

Follow **`specify/mvp-engine-validation.md`** for the full checklist and sign-off.

| Step | Action |
|------|--------|
| Automated (full) | `make check` with Docker running — includes `tests/*.rs` (LLM resolver, SSE, HTTP, RLS). |
| Automated (no Docker) | `make check` still runs `fmt` + `clippy`, then falls back to `cargo test --lib`. Use `make check-local` to skip the Docker probe entirely. |
| Manual (recommended once per env) | `make m4a-smoke` with **`M4A_LLM_API_KEY`** (see `scripts/dev/m4a-mvp-smoke.sh`), or `curl -N` on `POST /org/agents/<id>/complete/stream` (README). |
| Gate | **M4b** (MCP in product path + orchestration) starts **after M5**; keep M4a checklist green before shipping cockpit features that depend on the same API. |

## M5 (cockpit) — **current priority**

Tasks and phased scope: **`specify/tasks/005_milestone_5.md`**. Product wording: **`specify/spec.md` §5–7** (**M5a** vs **M5b**).

### M5a (cockpit v1 — shipped)

| Step | Action |
|------|--------|
| Automated (FE) | **`make fe-lint`**, **`make fe-build`** after substantive `frontend/` changes. |
| Manual (local) | **`make run`** + **`make fe-dev`**; **`.env`**: **`INVENTIV_CORS_ORIGINS`** must include the Vite origin; optional **`VITE_API_BASE`** in `frontend/.env.local`. |
| Smoke | Register org → login → **Registry** (provider + key, agent) → **Chat**: SSE **`delta`**, **`usage`**, **`meta.trace_id`**. |
| Spec Kit | Constitution **XIV** `[FE]`: loading/empty/error/success on main flows; **X** version in UI; **IX** telemetry still **partial** until M5b+ (see `005_milestone_5.md`). |

### M5b (sessions §5 — not shipped)

| Step | Action |
|------|--------|
| After **T5.7–T5.10** | Full `make check`; manual **two-user / two-browser** sharing smoke when session APIs exist; RLS assertions per org/group. |
| After **T5.11** | Spot-check **US.5**-style views once **persisted metrics** exist (**M4b** Phase 5+). |

### Later (cross-cutting)

Accessibility hardening, audit log export, **CI** job for `fe-lint` / `fe-build` (**Constitution VIII / XV**), full **IX** frontend telemetry coverage — as adopted by the team.

## M4b — MCP, persistence, full loop (**after M5**)

1. **After Phase 4 (MCP HTTP client)** — **`cargo test`** covers wiremock JSON-RPC stubs; **Manual**: point `McpHttpJsonRpcClient` at a real MCP HTTP endpoint (skill row); timeout and error paths.
2. **After Phase 5–6 (persistence + orchestration)** — **Manual + DB**: execution/metrics tables and RLS across orgs.

**Rule of thumb**: use **wiremock / CI** for LLM until you need a real provider; add **real MCP** in the live agent path when resuming **M4b** after M5.

## Cross-cutting

- **`make full`** (or CI): before tagging a release or promoting an artifact (**Constitution XV**).
- **Never** commit real production keys; use `.env` and secret stores for manual LLM tests.
