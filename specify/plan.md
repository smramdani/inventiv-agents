# Technical Plan: InventivAgents (v1.0.0 MVP)

## 1. Global Architecture: The Agentic Kernel
The Rust backend acts as an orchestration kernel. It manages the multi-tenant lifecycle of **Sessions** (persisted in **M5b**; **M5a** cockpit uses single-turn SSE per **`specify/spec.md` §6–8**) and the reasoning loop of **Mission-Driven Agents**.

## 2. Milestones and prioritised backlog

This section is the **Plan** artefact in Spec Kit terms: it ties **`specify/spec.md`** (product specification, **§2 Epics**, **§7 User Stories**) to **timeboxed milestones** (major / golden delivery targets — **M3**, **M4a**, **M5a**, …) and to **`specify/tasks/*.md`** (executable **technical tasks**). **Milestone** here means the same thing as a **story map swimlane** (`milestones[]` / `milestoneId` in `tools/storymap/backlog/*.json` v2). It is **not** the same word as **software release** in **Constitution XV** (immutable deployable artifact). Ordering below is **journey-first** (same narrative as **`spec.md` §2.1** and the **story map** `columns[]` left → right), then **milestone IDs** for engineering traceability.

**Canonical US ↔ tasks ↔ milestones matrix**: **`specify/traceability.md`** — update it when user stories move between milestones or when **`T###`** splits; then align this §2, task rows, and the story map JSON so nothing drifts.

### 2.0 Spec Kit vs Agile naming

**Technical task ids**: Work items use **globally unique** ids **`T001`**, **`T002`**, … (**`T###`**, three digits). The number does **not** embed the milestone: **schedule** is carried by the **`Milestone`** field on each task row and by the tables below. When planning changes, **keep the task id** and update **`Milestone` / `specify/traceability.md` / this §2 / story map** — do not renumber **Epics**, **US.x**, or existing **`T###`** (add new ids for new slices only).

| Spec Kit | Agile-friendly role |
| :--- | :--- |
| **`specify/spec.md`** | **Specification** — vision, **epics**, domain rules, **user stories**. |
| **`specify/plan.md` (this file)** | **Plan** — architecture §1, **milestone-ordered backlog** §2, components §3, security §4. |
| **`specify/traceability.md`** | **US ↔ `T###` ↔ milestone** — single place to edit before propagating to tasks + story map. |
| **`specify/tasks/*.md`** | **Technical tasks (`T###`)** — **not** user stories: small, verifiable **implementation** items (DB migration, endpoint, UI screen, test, doc) with acceptance and **XIV** layer tags. |
| **`tools/storymap/backlog/*.json`** | **Optional backlog view** — columns = epics, **swimlanes = milestones**, **one card per US per milestone**, **`refs`** = **`T###`** list. |

### 2.0.1 Epics, user stories, and technical tasks (one refinement chain)

This matches **Constitution XII / XVI** and avoids mixing Agile “story” wording with Spec Kit **technical tasks**.

1. **Epics (`spec.md` §2)** — Journey-sized **themes** on the backbone (story map **`columns[]`**). They order **short → long** horizon; they do **not** replace milestones or tasks.
2. **User stories (`spec.md` §7)** — **`US.1`–`US.15`**: each epic (`§2`) maps to **one or more** user stories (**§7.0**), and each **`US.x`** is bound to **exactly one milestone** (when a theme spans milestones, it is **split** into separate ids — e.g. `US.7` (M5a) → `US.11` (M5b) → `US.12` (M4b)). **Roadmap / prioritisation** in **this §2** (journey tables **§2.2–2.4**) lists **which user stories** matter **for which milestone**, grouped by **Epic** columns for readability. A user story is **not** a DB script or a single PR; it is the **“why”** slice of the product.
3. **Technical tasks (`T###` in `specify/tasks/*.md`)** — **Implementation** work: one migration, one route, one React flow, one contract test, etc. **Each `T###` row cites exactly one `US.x`** (**no `—`**) and that **`US.x`** lives in **exactly one milestone** (so each task inherits a unique Epic + milestone). **Several `T###` rows** may **realise the same `US.x`** within that milestone. Task titles should read as **engineering actions**, not as duplicate user-story marketing text.
4. **Story map cards** — Optional **visual** layer: **exactly one card per `userStoryId`** in the **Epic** column that owns that **US** (since each **US** belongs to one milestone, the swimlane is determined by the cited **US**); **`refs`** list **every `T###`** that implements that **US** (**`traceability.md`**).

**Anti-patterns**: (a) a **`T###`** line that is only a copy-paste of a full **US** without verifiable implementation scope; (b) inventing informal **`US`** ids instead of extending **`spec.md` §7** and **`T###`**; (c) planning “tasks” in **`plan.md` §2** without anchoring them to **`T###`** rows before merge (**XVI**).

### 2.0.2 Epic ↔ user stories (planning lens)

Same matrix as **`specify/spec.md` §7.0**; use when prioritising **by column** (story map) without opening the full spec. Every **`US.x`** below is **bound to exactly one milestone**.

| Epic | **`US.x`** (milestone) | Notes |
| :--- | :--- | :--- |
| **E-ONB** | **US.1** (M5a) | First org / repo / onboarding docs — **T033**, **T034**, **T046**. |
| **E-AUTH** | **US.2** (M5a) | Login, JWT, **`whoami` + role**. |
| **E-REG** | **US.3** (M5a), **US.4** (M5a), **US.5** (M5a), **US.6** (M5a) | Provider, skill, agent catalogs + **US.6** tenant-safe persistence/API. |
| **E-CHAT** | **US.7** (M5a), **US.11** (M5b), **US.12** (M4b) | Cockpit single-turn chat → persisted sessions / sharing → tool-augmented chat. |
| **E-USAGE** | **US.8** (M5a), **US.13** (M5b), **US.14** (M4b) | Last **usage** signal → owner dashboards → persisted runs / metrics ground truth. |
| **E-GOV** | **US.9** (M5b) | Invites, groups, access boundaries. |
| **E-ENG** | **US.10** (M5a), **US.15** (M4b) | Streaming LLM kernel (M4a) → MCP + orchestrator (M4b). |

### 2.1 Milestone overview (engineering timeline)

Chronological **delivery** order (dependencies). **Journey priority** for cockpit work is in §2.2–2.4.

| Milestone / tranche | Status | Focus | Task file |
| :--- | :--- | :--- | :--- |
| **M1 & M2** | Done | Identity, Auth, RLS, traceability, telemetry | Scaffold: `001_milestone_1.md`; shipped work in **CHANGELOG** / history. |
| **M3** | Done | Registry & entities (DB, domain, APIs) | `003_milestone_3.md` |
| **M4a** | Done | LLM + SSE MVP engine (no tools in product path) | `004_milestone_4.md` Phases 1–3 |
| **M5a** | Done | Sovereign Cockpit v1 (`frontend/`) | `005_milestone_5.md` (**T033**, **T034–T036**, **T037–T042**, **T043**, **T044–T045**, **T046**) |
| **M5b** | Planned | Persisted sessions, governance UI, sharing, metrics UX | `005_milestone_5.md` (**T047–T049**, **T050–T051**, **T052**, **T053–T054**) |
| **M4b** | After M5 | MCP in loop, runs/metrics persistence, orchestration | `004_milestone_4.md` Phases 4–6 |

### 2.2 M5a — journey-aligned backlog (shipped)

**Goal**: A new org can **onboard → authenticate → configure registry → chat → see usage** in the browser. **Story map**: one swimlane **M5a** across columns **`onboard` → `engine`** (engine column holds only cross-cutting / doc cards for this tranche).

| Step | Epic | `columns[].id` | User stories / capability | Typical tasks |
| :---: | :--- | :--- | :--- | :--- |
| 1 | **E-ONB** | `onboard` | **US.1** — first tenant + runnable monorepo (see story map **register org** card) | **T033**, **T034** |
| 2 | **E-AUTH** | `auth` | Login, JWT, **`whoami` + role** | **T035**, **T036** |
| 3 | **E-REG** | `registry` | **US.3**–**US.5** cockpit registry; **US.6** already satisfied by **M3** APIs | **T037–T042** |
| 4 | **E-CHAT** | `chat` | **US.7** single-turn SSE in cockpit (**M4a** route, **M5a** US) | **T043** |
| 5 | **E-USAGE** | `usage` | **US.8** slice — last **usage** from SSE | **T044**, **T045** |
| 6 | Cross-cutting | `governance` / `onboard` | **US.1** — M5a docs + FE toolchain | **T046** |

### 2.3 M5b — journey-aligned backlog (planned)

**Goal**: **Persisted conversations**, **govern** membership and groups, then **share**, then **owner reporting** on persisted metrics. Each step is a **distinct `US.x`** owned by **M5b** (per **`specify/spec.md` §7**); summary table in **`specify/spec.md` §7.16**.

| Step | Epic(s) | `columns[].id` | User stories / capability | Typical tasks |
| :---: | :--- | :--- | :--- | :--- |
| 1 | **E-CHAT** | `chat` | **US.11** — sessions schema + API + FE (list, resume, multi-turn SSE) | **T047–T049** |
| 2 | **E-GOV** | `governance` | **US.9** — invites and **groups** UI | **T050**, **T051** |
| 3 | **E-CHAT**, **E-GOV** | `chat`, `governance` | **US.11** — group-based **session sharing** (two-browser validation) | **T052** |
| 4 | **E-USAGE** | `usage` | **US.13** — dashboards wired to **US.14** metrics read API (M4b) | **T053**, **T054** |

#### 2.3.1 Step 1 — Persisted sessions foundation (**US.11**)

| | |
| :--- | :--- |
| **Launch when** | **M5a** stable; cockpit chat proven against **M4a** SSE. |
| **User-visible outcome** | Sessions **survive refresh**; org members see **their** session list; opening a session shows **history**; new messages **stream** in session context (multi-turn). |
| **Out of scope for this slice** | Invites/groups UI (**§2.3.2**); cross-user sharing (**§2.3.3**); persisted **token metrics** aggregates (**M4b** + **§2.3.4**). |
| **Tasks (order)** | **T047** (DB + RLS) → **T048** (HTTP API) → **T049** (React list / resume / SSE). |
| **Acceptance hints** | RLS: no cross-org session rows; API contracts documented in **README**; **`specify/testing-checkpoints.md`** M5b table after **T047–T049**. |

#### 2.3.2 Step 2 — Directory governance (**US.9**)

| | |
| :--- | :--- |
| **Launch when** | Session APIs sketched or done so **group ids** used in RLS policy are stable (**T047** schema alignment). |
| **User-visible outcome** | Owner/Admin can **invite** members and **manage groups** from the cockpit (happy + error paths, no secret leakage). |
| **Out of scope** | Full audit export, fine-grained ABAC (**later**). |
| **Tasks (order)** | **T050** → **T051** (either order acceptable if no hard dependency; prefer **T050** first for “users exist” flows). |
| **Acceptance hints** | RBAC matches **§3**; forms match **`005_milestone_5.md`** XIV **[FE]** gates. |

#### 2.3.3 Step 3 — Session sharing (**US.11** + **US.9**)

| | |
| :--- | :--- |
| **Launch when** | **T047–T049** + **T050–T051** done enough that two users can belong to the same **group**. |
| **User-visible outcome** | A session created by user A is **visible / continuable** by user B in the same group, per policy (two-browser smoke). |
| **Tasks** | **T052**. |
| **Acceptance hints** | **`specify/testing-checkpoints.md`**: two-browser, org/group RLS assertions. |

#### 2.3.4 Step 4 — Owner usage UI on persisted metrics (**US.13**)

| | |
| :--- | :--- |
| **Launch when** | **US.14** metrics **read API** + persisted data from **M4b** Phase 5 exist (or stub contract frozen). |
| **User-visible outcome** | **T053**: typed client + safe empty state; **T054**: Owner dashboard charts/tables on **real** persisted usage (not only last SSE turn). |
| **Tasks** | **T053** → **T054** (client before full dashboard if API lands incrementally). |
| **Note** | **M5b** steps **2.3.1–2.3.3** can proceed **without** **M4b**; this step is **blocked** on **§2.4** Phase 5 (and partially Phase 6 for end-to-end tool usage driving metrics). |

### 2.4 M4b — journey-aligned backlog (after M5)

**Goal**: **Production-grade engine** — MCP in the reasoning path, persisted runs/metrics, orchestrated streaming entry — delivering the **M4b** user stories **US.12** (tool-augmented chat), **US.14** (persisted runs / metrics ground truth) and **US.15** (MCP + orchestrator kernel). **Summary table**: **`specify/spec.md` §7.16**; **phases** below match **`004_milestone_4.md`**.

| Step | Epic(s) | User stories / capability | Task phases |
| :---: | :--- | :--- | :--- |
| 1 | **E-ENG**, **E-REG** | **US.15** — MCP **tools/call** in live loop (skills from **US.4**) | `004` Phase 4 |
| 2 | **E-ENG**, **E-CHAT** | **US.15** + **US.12** — orchestrator + tool-augmented streaming for users | `004` Phase 6 |
| 3 | **E-ENG**, **E-USAGE** | **US.14** — persisted runs / metrics + RLS + read API | `004` Phase 5 |

#### 2.4.1 Phase 4 — MCP ready in product architecture (**US.15**, supports **US.4**)

| | |
| :--- | :--- |
| **Outcome** | HTTP MCP client proven; **tool list / call** available to the orchestration layer; wiremock coverage in CI. |
| **Product tie-in** | Skills registered under **US.4** become **invocable** from the **reasoning path** (not only library code). |
| **Tasks** | **T024**–**T026** (integrate as needed with Phase 6). |
| **Depends on** | **M5** track unblocked for cockpit; library may exist early per **`004_milestone_4.md`**. |

#### 2.4.2 Phase 5 — Persisted runs and metrics (**US.14**)

| | |
| :--- | :--- |
| **Outcome** | Each completion (and later each tool step) leaves an **auditable row**: tokens, trace, org scope; **RLS** enforced on execution/metrics tables; an **org-scoped read API** exposes the data. |
| **Product tie-in** | **US.14** is the platform US that owns this slice; it unlocks the **US.13** owner dashboards in **§2.3.4**. |
| **Tasks** | **T027**, **T028**, **T029**. |
| **Unlocks** | **`plan.md` §2.3.4** (**T053**–**T054**). |

#### 2.4.3 Phase 6 — Orchestrated streaming loop (**US.15**, **US.12**)

| | |
| :--- | :--- |
| **Outcome** | Single **orchestrated** service path: LLM + optional MCP tools + validation + SSE to client; documented entry route. |
| **Product tie-in** | **US.15** is the platform-side US (kernel + orchestrator); **US.12** is the user-facing tool-augmented chat experience that ships on top of it (with **US.5** agents and **US.4** skills). |
| **Tasks** | **T030**, **T031**, **T032**. |
| **Depends on** | Phases **4** and **5** for meaningful end-to-end validation. |

## 3. Component Design

### LLM Abstraction Layer (M4a — shipped)
- OpenAI-compatible APIs; SSE streaming for the cockpit and API clients.

### MCP & tool loop (M4b — **after M5**)
- Library: `McpHttpJsonRpcClient` in `backend/src/infrastructure/mcp/` (`tools/list`, `tools/call`) exists; **product integration** (reasoning loop, live SSE + tools) follows **`004_milestone_4.md`** Phases 4–6.
- Full loop + persisted metrics: **`004_milestone_4.md`** Phases 5–6.

### Reasoning Loop Logic
- **M4a (shipped)**: Single-turn LLM over SSE; token counts in SSE `usage`.
- **M4b (post-M5)**: `Reasoning` → tool selection → MCP execution → validation → response; cost tracking persisted per step where required.

## 4. Security & Safety
- **Isolation**: Row Level Security (RLS) for every entity (Skills, Agents, Sessions).
- **Sandboxing (M4b+)**: MCP tools restricted by agent/skill policy once the tool loop ships (**after M5**).

## 5. Spec ladder: epics, milestones, user stories, technical tasks, story map

Artifacts follow one **top-down spine** (**Constitution XII**, **XVI**); see **`specify/spec.md` §10**.

| Layer | Artifact | Role |
| :--- | :--- | :--- |
| **Epics** | `specify/spec.md` §2 | **Journey backbone** (§2.1 order); each maps to a **story map column** (`epicId`). |
| **User stories** | `specify/spec.md` §7 (**§7.0** epic map + **§7.1–7.15** narratives) | Stable **US.1–US.15** outcomes; **each US belongs to exactly one milestone** (split into a new `US.x` if the theme also targets another milestone — see `traceability.md`). |
| **Milestone backlog** | `specify/plan.md` §2 | **Journey-ordered** steps inside **M5a / M5b / M4b**, plus §2.1 engineering timeline. |
| **Traceability** | `specify/traceability.md` | **US ↔ `T###` ↔ milestone** matrix; edit before task files and story map. |
| **Technical tasks** | `specify/tasks/*.md` | **Elementary** implementation items (**`T###`** ids per §2.0); cite **Epic**, **US**, **`Milestone`**, and **`plan.md` §2** step. |
| **Story map (optional)** | `tools/storymap/backlog/*.json` | **Columns = epics**, **lanes = milestones**, **exactly one card per `userStoryId`** (lane derived from the cited **US**), **`refs`** = all **`T###`** that implement that **US**. |

- **Near term**: Decompose the active **§2** step into **tasks** before coding; update **spec / traceability / plan / tasks / story map** together when scope shifts.
- **Later milestones**: Keep **§2.4** high-level until **M4b** is scheduled; then expand tasks in **`004_milestone_4.md`**.
