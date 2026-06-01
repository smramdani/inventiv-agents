# Traceability: Milestones · User stories · Tasks

**Purpose**: One place to see how **milestones** (same concept as **golden / major delivery targets** in `specify/spec.md` §8), **`US.1`–`US.15`** user stories (`specify/spec.md` §7), and globally unique **`T###`** tasks (`specify/plan.md` §2.0) fit together. **Update this file first** when planning changes which **user stories** belong to which **milestone**, then align **`specify/plan.md` §2**, **`specify/tasks/*.md`** (**Milestone** + **User story** on each task row), and **`tools/storymap/backlog/*.json`** (`milestoneId`, `userStoryId`, `refs`) so the **story map** cannot drift.

## Terminology (US vs technical task)

| Layer | Id / artefact | What it is | Typical granularity |
| :--- | :--- | :--- | :--- |
| **Epic** | **`E-*`**, `spec.md` §2, story map **`columns[]`** | Journey **theme** for ordering and personas | Months / horizons; not a PR |
| **User story** | **`US.1`–`US.15`**, `spec.md` §7 | **Product value** for a persona (“want / so that”) **bound to exactly one milestone** | Epic-derived; if a theme spans milestones it is **split into separate `US.x`** |
| **Technical task** | **`T###`**, `specify/tasks/*.md` | **Implementation action** (DB, API, domain, FE, test, doc) | PR-sized, verifiable; **many per US**, but the US (and therefore Epic + milestone) is unique per task |
| **Story map card** | JSON card + `refs` | **Communications / planning** — **one card per `userStoryId`** (= one card per US per milestone) | **`refs`** list **all `T###`** for that **US** in that milestone |

Each **`T###`** row names **exactly one** **`User story` (`US.1`–`US.15`)** (**`specify/constitution.md`** **XII**). Because every **US** is bound to a single milestone, citing the **US** also pins the task to that **Epic** + **milestone**.

## Epics → user stories (same as `spec.md` §7.0)

| Epic | **`US.x`** (milestone) |
| :--- | :--- |
| **E-ONB** | **US.1** (M5a) |
| **E-AUTH** | **US.2** (M5a) |
| **E-REG** | **US.3** (M5a), **US.4** (M5a), **US.5** (M5a), **US.6** (M5a) |
| **E-CHAT** | **US.7** (M5a), **US.11** (M5b), **US.12** (M4b) |
| **E-USAGE** | **US.8** (M5a), **US.13** (M5b), **US.14** (M4b) |
| **E-GOV** | **US.9** (M5b) |
| **E-ENG** | **US.10** (M5a), **US.15** (M4b) |

**Rules**

- **Milestone** = timeboxed target (M3, M4a, M4b, M5a, M5b, …). Not the same word as **software release** in **Constitution XV** (immutable deployable artifact).
- **One US ↔ one milestone**: a **`US.x`** is bound to a single milestone. If a product theme spans several milestones, **split** it into separate **`US.x`** (e.g. `US.7` cockpit chat in **M5a**, `US.11` persisted sessions in **M5b**, `US.12` tool-augmented chat in **M4b**). When you reprioritise, **move the `US.x`** to the new milestone — do not duplicate it across milestones, and do not renumber existing **`US.x`** or **`T###`** (add new ids only for new slices).
- **One card per `userStoryId`**: The backlog viewer enforces **exactly one card** per **`userStoryId`**, and that card lives on the swimlane of the US’s milestone. Several **`T###`** rows implement the same **`US.x`**; they are **listed on that card** (`refs`), not as separate cards.
- Every **task** row in `specify/tasks/*.md` must name **Milestone**, **Epic**, and **exactly one** **User story** (**`US.1`–`US.15`**). The **Epic** + **Milestone** must match the ones bound to that **US** here.

---

## 1. Milestones → User stories (planning scope)

| Milestone | Status (high level) | User stories in scope | Task files |
| :--- | :--- | :--- | :--- |
| **M3** | Done (registry) | **US.3**, **US.4**, **US.5**, **US.6** | `003_milestone_3.md` |
| **M4a** | Done (SSE engine, no tools in product path) | **US.7**, **US.8**, **US.10** | `004_milestone_4.md` Phases 1–3 |
| **M5a** | Done (cockpit v1) | **US.1**, **US.2**, **US.3**, **US.4**, **US.5**, **US.6**, **US.7**, **US.8**, **US.10** | `005_milestone_5.md` (cockpit) + `004_milestone_4.md` Phases 1–3 (kernel) |
| **M5b** | Planned | **US.9**, **US.11**, **US.13** | `005_milestone_5.md` |
| **M4b** | After M5 | **US.12**, **US.14**, **US.15** | `004_milestone_4.md` Phases 4–6 |

> **M3** ships the registry data layer that the **M5a** product user stories (**US.3**–**US.6**) depend on; the data tasks **T006**–**T014** are scoped to **US.3**–**US.6** (which are themselves **M5a** product US owning the cockpit + API surface).

### 1.1 Dev planning matrix (milestone → US → `T###`)

Per the **one US ↔ one milestone** rule, the table below shows the **complete `US.x` set per milestone** and the **typical `T###`** that ship each one. Detail: **`spec.md` §7.16**, **`plan.md` §2.3.1–2.3.4** and **§2.4.1–2.4.3**.

| Milestone | **`US.x`** | Slice (one card on the story map) | Typical **`T###`** |
| :--- | :--- | :--- | :--- |
| **M5a** | **US.1** | Onboard org + monorepo | **T033**, **T034**, **T046** |
| **M5a** | **US.2** | Login + JWT + role gates | **T035**, **T036** |
| **M5a** | **US.3** | Provider catalog (data + cockpit) | **T006**, **T010**, **T037**, **T038** |
| **M5a** | **US.4** | Skill catalog (data + cockpit) | **T007**, **T011**, **T039**, **T040** |
| **M5a** | **US.5** | Agent catalog (data + cockpit) | **T008**, **T012**, **T041**, **T042** |
| **M5a** | **US.6** | Tenant-safe registry persistence + API | **T009**, **T013**, **T014** |
| **M5a** | **US.7** | Cockpit single-turn SSE chat | **T016**, **T021**–**T023**, **T043** |
| **M5a** | **US.8** | Cockpit last-turn `usage` signal | **T020**, **T044**, **T045** |
| **M5a** | **US.10** | Streaming LLM kernel (M4a) | **T015**, **T017**–**T019** (scaffold **T001–T005** in `001_milestone_1.md` is **historical / superseded** — see note in §2 below) |
| **M5b** | **US.11** | Persisted sessions + resume + sharing | **T047**, **T048**, **T049**, **T052** |
| **M5b** | **US.9** | Invites + groups | **T050**, **T051** |
| **M5b** | **US.13** | Owner dashboards on persisted metrics | **T053**, **T054** |
| **M4b** | **US.15** | MCP `tools/call` in loop + orchestrator + persisted run linkage | **T024**–**T026**, **T030**–**T032** |
| **M4b** | **US.14** | Persisted runs / token metrics + RLS + read API | **T027**, **T028**, **T029** |
| **M4b** | **US.12** | Tool-augmented orchestrated chat (user-facing) | (ships on top of **US.15**; no new `T###` beyond Phase 6) |

---

## 2. User stories → Tasks (`T###`)

Exhaustive mapping is on **each task row** in `specify/tasks/*.md`. Each **US** lives in **one milestone**; the milestone is shown in parentheses.

### US.1 — Organization onboarding · **E-ONB** · **M5a**

**T033**, **T034**, **T046**

### US.2 — Authenticated identity · **E-AUTH** · **M5a**

**T035**, **T036**

### US.3 — Sovereign LLM provider catalog · **E-REG** · **M5a**

**T006**, **T010**, **T037**, **T038**

### US.4 — MCP-backed skill catalog · **E-REG** · **M5a**

**T007**, **T011**, **T039**, **T040**

### US.5 — Mission-driven agent catalog · **E-REG** · **M5a**

**T008**, **T012**, **T041**, **T042**

### US.6 — Tenant-safe registry platform · **E-REG** · **M5a**

**T009**, **T013**, **T014**

### US.7 — Streamed single-turn agent chat (cockpit) · **E-CHAT** · **M5a**

**T016**, **T021**, **T022**, **T023**, **T043**

### US.8 — Last-turn usage signal in the cockpit · **E-USAGE** · **M5a**

**T020**, **T044**, **T045**

### US.9 — People & access structure · **E-GOV** · **M5b**

**T050**, **T051**

### US.10 — Streaming LLM execution kernel · **E-ENG** · **M5a**

**T015**, **T017**, **T018**, **T019** (M4a kernel — `004_milestone_4.md` Phases 1–3).

> **Historical note**: `specify/tasks/001_milestone_1.md` (**T001**–**T005**) is the early bootstrap checklist (`Cargo.toml`, base folders, `Agent` trait, hello-world Axum, base LLM client). It is marked **superseded** by the shipped stack under `backend/` and later milestones (M3 / M4a / M5a); it is **not counted** as live `US.10` work in the story map and not required for any `US.10` acceptance check.

### US.11 — Persisted multi-turn agent sessions + sharing · **E-CHAT** · **M5b**

**T047**, **T048**, **T049**, **T052**

### US.12 — Tool-augmented orchestrated agent chat · **E-CHAT** · **M4b**

User-facing slice that ships on top of **US.15**. No dedicated **`T###`** beyond `004` Phase 6 (**T030**–**T032**); validation captured in **`specify/testing-checkpoints.md`** M4b.

### US.13 — Owner usage dashboards on persisted metrics · **E-USAGE** · **M5b**

**T053**, **T054** (consume the **US.14** read API)

### US.14 — Persisted runs and token metrics ground truth · **E-USAGE** · **M4b**

**T027**, **T028**, **T029**

### US.15 — MCP execution + orchestrated streaming kernel · **E-ENG** · **M4b**

**T024**, **T025**, **T026**, **T030**, **T031**, **T032**

---

## 3. Story map card fields (no drift)

For each card in `tools/storymap/backlog/*.json` (format **v2**):

| Field | Must match |
| :--- | :--- |
| `milestoneId` | A milestone in §1 and in `milestones[].id`, **equal to the milestone bound to `userStoryId` in §2** above. |
| `userStoryId` | One of **`US.1`** … **`US.15`** from **`spec.md` §7**. **Exactly one card** per **`userStoryId`** per backlog file (= one card per US per milestone). The **US id** prefixes the card subtitle in the viewer; full **story** + **task list** in the **modal**. |
| `refs` | Include **every `T###`** that implements this **US** for this **milestone**; labels **`T###`** must appear in **`specify/tasks/*.md`**. |

When you **add or split tasks**, update **`refs`** on the **single** card for that **`userStoryId`**; when you **reprioritise**, **move the `US.x` to a new milestone** (do not duplicate it across milestones, and do not renumber). If the new milestone needs a different scope, **introduce a new `US.x`** and split tasks accordingly, then update `plan.md` §2, the task rows, and the story map JSON in the same change set.
