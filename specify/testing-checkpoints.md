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

## M4a — MVP engine (**no tools, no MCP**) — **current priority**

Follow **`specify/mvp-engine-validation.md`** for the full checklist and sign-off.

| Step | Action |
|------|--------|
| Automated (full) | `make check` with Docker running — includes `tests/*.rs` (LLM resolver, SSE, HTTP, RLS). |
| Automated (no Docker) | `make check` still runs `fmt` + `clippy`, then falls back to `cargo test --lib`. Use `make check-local` to skip the Docker probe entirely. |
| Manual (recommended once per env) | `curl -N` on `POST /org/agents/<id>/complete/stream` with a real **test** provider key (see README). |
| Gate | **Do not** start MCP client / tool orchestration work until MVP checklist is signed off (roadmap **M4b**). |

## M4b — after MVP (MCP, persistence, full loop)

1. **After Phase 4 (MCP)** — **Manual**: local stub MCP or sandbox; timeout and error paths.
2. **After Phase 5–6 (persistence + orchestration)** — **Manual + DB**: execution/metrics tables and RLS across orgs.

**Rule of thumb**: use **wiremock / CI** for LLM until you need a real provider; add **real MCP** only when implementing M4b.

## M5 (cockpit) — recommended manual gates

- After first **authenticated UI** path: smoke in browser (login, empty states).
- After **session sharing**: two browsers / two users, RLS checks.
- Before release: **accessibility**, **audit log** export, and **cost dashboard** spot-check against known test data.

## Cross-cutting

- **`make full`** (or CI): before tagging a release or promoting an artifact (**Constitution XV**).
- **Never** commit real production keys; use `.env` and secret stores for manual LLM tests.
