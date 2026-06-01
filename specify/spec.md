# Specification: InventivAgents Platform (v1.0.0 MVP)

## 1. Product Vision
An open-source B2B agentic AI platform (AGPL-3.0) enabling SMEs to deploy safe, secure, and sovereign AI. The platform provides a "Design-to-Cost" model where companies control their own models (Managed Cloud APIs) and audit all usage.

## 2. Epics (journey backbone · personas)

**Epics** are **journey-sized** outcomes: ordered **left → right** like the **story map backbone** (`tools/storymap/backlog/*.json` → `columns[]`). They bridge **short-term** cockpit delivery (**M5a**), **next** collaboration and sessions (**M5b**), and **long-term** engine depth (**M4b**, headless **M4a**). They align with **personas** (§3) and with **`specify/plan.md` §2** (milestone-prioritised user stories). **User stories** (§7) and **technical tasks** (`specify/tasks/*.md`) stay traceable to **at least one** epic. **US ↔ `T###` ↔ milestone** matrix: **`specify/traceability.md`**.

### 2.1 Journey order (short-term → long-term)

Read top to bottom as **first value**, then **repeatable daily use**, then **platform depth**:

1. **E-ONB — Onboard** — First org and runnable product surface (Owner, maintainer).
2. **E-AUTH — Auth & identity** — Durable client session and **role**-aware UI (Owner, Admin, User).
3. **E-REG — Registry** — Sovereign **providers**, **skills**, **agents** the org may run (Admin, Owner; User reads where allowed).
4. **E-CHAT — Agent conversations** — Streamed turns (**M5a**); **persisted sessions** and collaboration (**M5b**); orchestrated tool loop later (**M4b**).
5. **E-USAGE — Usage & cost** — Last-turn **usage** (**M5a**); dashboards on persisted data (**M5b** + **M4b**).
6. **E-GOV — People & access** — Invites and **groups** so sharing and org structure scale (**M5b** onward).
7. **E-ENG — Platform engine** — Contracts, LLM adapters, **SSE API** (**M4a**); **MCP**, runs/**metrics**, orchestration (**M4b**); migrations for sessions (**M5b**).

### 2.2 Epic reference (story map · horizons)

| Order | ID | Epic | Story map `columns[].id` | Primary personas | Shipped / near (typical) | Next | Later |
| :---: | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 1 | **E-ONB** | Onboard | `onboard` | Owner, maintainer | **M5a** — register org, repo layout | — | — |
| 2 | **E-AUTH** | Auth & identity | `auth` | Owner, Admin, User | **M5a** — login, JWT, `whoami` + **role** | **M5b** — session-aware client | — |
| 3 | **E-REG** | Registry | `registry` | Admin, Owner, User | **M5a** + **M3** — create/list providers, skills, agents | **M5b+** — edit/delete, agent–skill links | **M4b** — skills in live loop |
| 4 | **E-CHAT** | Agent conversations | `chat` | User, Admin, Owner | **M5a** — `US.7` single-turn SSE chat in cockpit | **M5b** — `US.11` persisted sessions / resume / share | **M4b** — `US.12` tool-augmented orchestrated chat |
| 5 | **E-USAGE** | Usage & cost | `usage` | Owner | **M5a** — `US.8` last-turn `usage` from SSE | **M5b** — `US.13` dashboards on persisted metrics | **M4b** — `US.14` persisted runs / metrics ground truth |
| 6 | **E-GOV** | People & access | `governance` | Admin, Owner | Optional **M5a** docs only | **M5b** — `US.9` invites, groups, sharing rules | Policies as product grows |
| 7 | **E-ENG** | Platform engine | `engine` | maintainer, Admin | **M5a** — `US.10` streaming LLM kernel (M4a) | (kernel-only milestones) | **M4b** — `US.15` MCP client, runs/metrics tables, orchestrator |

**Traceability (Spec Kit + Agile)**: **Epic (this §)** → **User story (§7)** → **Milestone order (`plan.md` §2)** → **Technical task (`specify/tasks/*.md`, `T###`)** → code. The **story map** places **exactly one card per `userStoryId`** (**Constitution v1.8.0** — **XII / XVI**): each **`US.x`** is bound to **one milestone**, so **`milestoneId`** + **`columnId`** (= owning **Epic**) are determined by the cited **US**; **`refs`** lists **all `T###`** that implement that **US**.

## 3. Organization Structure & Users (Done)
### Roles (RBAC)
- **Owner**: Total control over the organization, providers, and access.
- **Admin**: Manages users, groups, and creates Agent Templates (Skills).
- **User**: Creates and uses AI sessions based on authorized templates.

### Group Management
- Groups serve as the boundary for sharing Agent Templates and Sessions.

## 4. Sovereign Provider Management (v1.0.0)
- **Managed APIs**: Admins register LLM providers (OVH Cloud AI, Scaleway, Azure OpenAI) via API URL and Keys.
- **Usage Tracking (target)**: Systematically log token consumption in the **`metrics`** table for cost analysis. Persisted ground truth ships as **US.14** (M4b); Owner dashboards on top of it ship as **US.13** (M5b).
- **Usage in the cockpit (M5a, shipped)**: The web client surfaces the **last `usage` object** from the SSE stream for the current turn (read-only, client-side). This is the **`US.8` (M5a) slice** of the broader usage theme; persistence (**`US.14`**, M4b) and Owner dashboards (**`US.13`**, M5b) are separate user stories — see §7.16 and §8.

## 5. Skills, Agents & MCP (v1.0.0)
### MCP Skills
- **Definition**: Atomic unit of capability connected via Model Context Protocol.
- **Types**: Internal Knowledge Base (PDF Search), SQL connectors, or custom external MCP servers.

### Mission-Driven Agents
- **Definition**: Specialized AI personas with a defined **Mission**, **Persona**, and a **Toolbelt** (0..N Skills).
- **Control**: Admins decide which Agents have access to which Skills.

## 6. Agentic Sessions & Collaboration (v1.0.0)

**Target product (this section)**: A durable **Session** is the multi-tenant execution space for an agent conversation: message history, optional document context (RAG), and **sharing within a Group** for collaboration.

**Phased delivery (aligned with milestone 5 tasks)** — per **Constitution XII** and **XVI** (spec/plan/tasks/story map stay in sync; granularity ladder in **§10**):

- **M5a (shipped — “Sovereign Cockpit” v1)**: Authenticated **web client** with registry CRUD (create/list) for providers, skills, and agents, plus **single-turn SSE chat** against the existing M4a endpoint **`POST /org/agents/:id/complete/stream`** (SSE). **M5b** adds persisted sessions, server-side history, and group-based sharing per this section.
- **M5b (planned)**: Persisted **sessions** (schema + RLS under **`backend/migrations/`**), message history, sharing rules per **§3 Groups**, and cockpit UX for session list/resume/share. Tracked in **`specify/tasks/005_milestone_5.md`**.

## 7. User Stories (MVP)

**Stable ids**: **`US.1`**–**`US.15`**. Each id is a **product outcome** (persona / value), **not** a technical task, and lives in **exactly one milestone**. When a product theme spans several milestones, it is **split into separate user stories** (`US.7` cockpit chat in **M5a**, `US.11` persisted sessions in **M5b**, `US.12` tool-augmented chat in **M4b**, etc.). **Epics** (`§2`) decompose into **one or more** user stories in the table below. **Scheduling** by milestone is in **`specify/plan.md` §2** and **`specify/traceability.md`**. Task-level traceability rules: **`specify/constitution.md`** **XII** / **XVI**.

### 7.0 Epics → user stories (authoritative)

**Rule**: a **user story** belongs to **exactly one milestone**. When a product theme spans several milestones, it is **split into separate user stories** (one per milestone). The table below names that split.

| Epic | User stories | Milestone | Summary |
| :--- | :--- | :--- | :--- |
| **E-ONB** | **US.1** | **M5a** | First organization and runnable product entry (register org, cockpit shell, onboarding docs). |
| **E-AUTH** | **US.2** | **M5a** | Authenticated session with **role**-aware access for org members. |
| **E-REG** | **US.3** | **M5a** | Sovereign LLM provider catalog (cockpit list / create + M3 RLS). |
| **E-REG** | **US.4** | **M5a** | MCP-backed skill catalog (cockpit list / create). |
| **E-REG** | **US.5** | **M5a** | Mission-driven agent catalog (cockpit list / create). |
| **E-REG** | **US.6** | **M5a** | Tenant-isolated registry persistence + org-scoped registry API (M3 RLS, consumed by **M5a** cockpit). |
| **E-CHAT** | **US.7** | **M5a** | **Streamed single-turn agent chat in the cockpit** (M4a SSE route). |
| **E-CHAT** | **US.11** | **M5b** | **Persisted multi-turn agent sessions + group sharing**. |
| **E-CHAT** | **US.12** | **M4b** | **Tool-augmented orchestrated agent chat** (MCP in the live loop). |
| **E-USAGE** | **US.8** | **M5a** | **Last-turn `usage` signal in the cockpit** (read-only, from SSE). |
| **E-USAGE** | **US.13** | **M5b** | **Owner usage dashboards on persisted metrics** (consumes the **US.14** read API). |
| **E-USAGE** | **US.14** | **M4b** | **Persisted runs and token metrics ground truth** (RLS, read API). |
| **E-GOV** | **US.9** | **M5b** | Invites, groups, and sharing/access boundaries for the org directory. |
| **E-ENG** | **US.10** | **M5a** | **Streaming LLM execution kernel** (ports, LLM adapter, SSE wiring shipped with **M4a**). |
| **E-ENG** | **US.15** | **M4b** | **MCP execution + orchestrated streaming kernel** (tools/call in loop, orchestrator service, persisted runs wiring). |

### 7.1 **US.1** — Organization onboarding (Owner / maintainer) · **E-ONB**

| | |
| :--- | :--- |
| **As a** | Owner (or maintainer onboarding the tenant) |
| **I want** | to create my **organization** once and land in a **documented, runnable** cockpit + API layout |
| **So that** | every subsequent user and dataset is scoped to **`organization_id`** and contributors share one dev/build path |

### 7.2 **US.2** — Authenticated identity (Owner / Admin / User) · **E-AUTH**

| | |
| :--- | :--- |
| **As a** | Org member |
| **I want** | to **sign in** and have the client resolve my **role** for API and UI gates |
| **So that** | Owner, Admin, and User capabilities stay enforced consistently |

### 7.3 **US.3** — Sovereign LLM provider catalog (Admin / Owner) · **E-REG**

| | |
| :--- | :--- |
| **As a** | Admin or Owner |
| **I want** | to register and manage **managed LLM providers** (URL, credentials) in my org catalog |
| **So that** | agents resolve **sovereign** endpoints under our keys and residency choices |

### 7.4 **US.4** — MCP-backed skill catalog (Admin / Owner) · **E-REG**

| | |
| :--- | :--- |
| **As a** | Admin |
| **I want** | to register **Skills** (including MCP-backed) the org can attach to agents |
| **So that** | governed tool surfaces are reusable, not one-off integrations |

### 7.5 **US.5** — Mission-driven agent catalog (Admin / Owner) · **E-REG**

| | |
| :--- | :--- |
| **As a** | Admin |
| **I want** | to define **Agents** (mission, persona, LLM choice, allowed skills) |
| **So that** | users run consistent, policy-bound assistants (e.g. HR Agent) |

### 7.6 **US.6** — Tenant-safe registry platform (Admin / Owner) · **E-REG** (+ **RLS** as product guarantee)

| | |
| :--- | :--- |
| **As a** | Admin or Owner |
| **I want** | **providers, skills, and agents** persisted with **RLS** and served through **org-scoped** repository and HTTP APIs |
| **So that** | no tenant can read or mutate another tenant’s registry rows |

### 7.7 **US.7** — Streamed single-turn agent chat (cockpit) · **E-CHAT** · **M5a**

| | |
| :--- | :--- |
| **As a** | User |
| **I want** | to send a prompt to a configured agent and receive a **streamed** response in the cockpit |
| **So that** | I can use sovereign agents from the browser without bespoke integrations |

### 7.8 **US.8** — Last-turn usage signal in the cockpit (Owner) · **E-USAGE** · **M5a**

| | |
| :--- | :--- |
| **As a** | Owner |
| **I want** | the **last `usage` payload** from the SSE stream surfaced in the cockpit |
| **So that** | I get an immediate, read-only signal of token consumption per turn |

### 7.9 **US.9** — People & access structure (Admin / Owner) · **E-GOV**

| | |
| :--- | :--- |
| **As a** | Admin or Owner |
| **I want** | to manage **invites**, **groups**, and membership boundaries that sessions and sharing rely on |
| **So that** | collaboration scales without breaking tenant isolation intent |

### 7.10 **US.10** — Streaming LLM execution kernel (maintainer / platform) · **E-ENG** · **M5a**

| | |
| :--- | :--- |
| **As a** | Maintainer or platform engineer acting for product delivery |
| **I want** | a secure, observable **runtime** for **LLM** streaming behind the public APIs (ports, adapter, SSE, TraceID) |
| **So that** | cockpit and API clients share one tested kernel for **single-turn** completions |

### 7.11 **US.11** — Persisted multi-turn agent sessions + sharing (User) · **E-CHAT** · **M5b**

| | |
| :--- | :--- |
| **As a** | User |
| **I want** | to **resume** prior conversations and **share** them with my **group** when policy allows |
| **So that** | work with agents survives reloads and can be continued by teammates |

### 7.12 **US.12** — Tool-augmented orchestrated agent chat (User) · **E-CHAT** · **M4b**

| | |
| :--- | :--- |
| **As a** | User |
| **I want** | streamed completions that may **call governed tools (MCP skills)** through an **orchestrated** path |
| **So that** | configured agents can act on data and systems within the policy boundary |

### 7.13 **US.13** — Owner usage dashboards on persisted metrics (Owner) · **E-USAGE** · **M5b**

| | |
| :--- | :--- |
| **As a** | Owner |
| **I want** | dashboards on **persisted metrics** (tokens, optional cost) for my org |
| **So that** | I can steer budget and governance beyond a single SSE turn |

### 7.14 **US.14** — Persisted runs and token metrics ground truth (platform) · **E-USAGE** · **M4b**

| | |
| :--- | :--- |
| **As a** | Maintainer / Owner |
| **I want** | **execution runs and token metrics persisted with RLS** and exposed via an org-scoped read API |
| **So that** | reporting reflects real, auditable multi-tenant usage |

### 7.15 **US.15** — MCP execution + orchestrated streaming kernel (platform) · **E-ENG** · **M4b**

| | |
| :--- | :--- |
| **As a** | Maintainer or platform engineer |
| **I want** | the engine to run **MCP `tools/call`** in the reasoning loop, with an **orchestrator service** and **persisted run linkage** behind a documented streaming entry route |
| **So that** | tool-using agents from **US.5** / **US.4** are exercised consistently across cockpit and API clients |

### 7.16 Delivery summary by milestone (planning view)

Use this table when **ordering dev work**: it lists the **single milestone** that owns each **US** and the **typical `T###`** that ship it. **Canonical order** for the next increments: **`plan.md` §2.3.1–2.3.4** then **`§2.4`**. **Canonical** US↔`T###`↔milestone matrix: **`specify/traceability.md`**.

| **Milestone** | **User stories** | **What it ships (one slice each)** | **Typical `T###`** |
| :--- | :--- | :--- | :--- |
| **M5a** (shipped) | **US.1**, **US.2**, **US.3**, **US.4**, **US.5**, **US.6**, **US.7**, **US.8**, **US.10** | Sovereign Cockpit v1: register org / login + role-aware UI, **list + create** providers / skills / agents, **single-turn SSE** chat, **last-turn `usage`** display; org-scoped registry APIs + RLS already shipped via **M3**; **streaming LLM kernel** (M4a) backs `US.7`. | Cockpit: **T033**, **T034–T036**, **T037–T042**, **T043**, **T044–T045**, **T046**. M3 data layer (US.3–US.6): **T006–T014**. M4a kernel: **US.10** (**T015**, **T017–T019**), **US.7** (**T016**, **T021–T023**), **US.8** (**T020**). |
| **M5b** (next) | **US.9**, **US.11**, **US.13** | **Persisted multi-turn sessions** + **resume** + **group sharing**, **invites + groups** cockpit, and **Owner dashboards** built on the **US.14** read API. | **US.11**: **T047–T049**, **T052**. **US.9**: **T050–T051**. **US.13**: **T053–T054**. |
| **M4b** (after M5) | **US.12**, **US.14**, **US.15** | **Tool-augmented orchestrated chat** (MCP in the live loop), **persisted runs + token metrics** with RLS + read API, **orchestrator service** + persisted run linkage. | **US.15** MCP loop: **T024–T026**; **US.14** runs/metrics + RLS + read API: **T027–T029**; **US.15** orchestrator + persistence wiring: **T030–T032**. **US.12** (user-facing) ships on top of US.15 (no extra `T###`). |

## 8. Milestones M4 / M5 / M4b (technical path, v1.0.0)

**M4a (shipped)**: Headless **agentic engine** without tool execution in the product path: OpenAI-compatible **LLM**, org-scoped **provider resolution**, **SSE** completion, **TraceID** / structured logging. Minimal **US.7** slice for API clients (via **US.10** kernel tasks).

**M5 (current priority)**, split per §6:

- **M5a (shipped)**: **Sovereign Cockpit v1** — `frontend/` Vite/React SPA: **register / login**, **registry** (create + list providers, skills, agents; Owner/Admin), **SSE chat** (single-turn) + **last-turn usage** display, **CORS** and **`whoami` role** for RBAC in the UI. **M5b** adds persisted sessions, group sharing, and **`US.13`** Owner dashboards built on the **`US.14`** persisted-metrics read API. Tasks: **`specify/tasks/005_milestone_5.md`** (atomic **T033**, **T034–T036**, **T037–T042**, **T043**, **T044–T045**, **T046** for **M5a**).
- **M5b (next within M5)**: Persisted **sessions**, history, and **collaboration** as in §6; extends the same milestone file before marking “M5 complete” against §6–7.

**M4b (after M5)**: **Execution kernel** (**§7.15** — `US.15`) — MCP **tools/call** in the live loop, persisted runs/metrics (**RLS** — `US.14`, §7.14), orchestrated streaming — `specify/tasks/004_milestone_4.md` Phases **4–6**. HTTP MCP **client** in `backend/src/infrastructure/mcp/` supports **M4b** product integration. The user-facing tool-augmented chat experience that ships on top is **`US.12`** (§7.12).

**M4a validation checklist**: `specify/mvp-engine-validation.md`.

## 9. Open Source & License
- **License**: AGPL-3.0.
- **Language**: English.

## 10. Specification ladder, backlog granularity, and story map

This section summarizes **Constitution XII** and **XVI** for spec readers (the constitution remains authoritative).

- **Top-down (strategic → particular)**: **§1–6** describe *what* the platform delivers. **§2 Epics** (§2.1 order, §2.2 table) name the **user journey backbone** aligned with the **story map columns**. **§7 User Stories** (**`US.1`–`US.15`**) anchor *who* gains *value* (**outcomes**, not tickets); each **US** belongs to **exactly one milestone** (when a theme crosses milestones, it splits into separate **US** ids). **§7.16** summarises the milestone → US assignment for planning. **§8 Milestones** name **major delivery targets** (M3, M4a, M5a, …). **`specify/plan.md` §2** lists **milestone-prioritised** work in **journey / Epic order**, then links to **technical** execution. **`specify/traceability.md`** is the **canonical** **US ↔ `T###` ↔ milestone** matrix. **`specify/tasks/*.md`** holds **`T###`** implementation rows under **`specify/constitution.md`** **XII** / **XVI**. Lower layers must **trace upward** to **§7** and **§2**. See **`plan.md` §2.0.1** for the **Epic → user story → technical task** chain.
- **Time horizon**: **Shipped** capability is described at **testable** detail. **Planned** work may stay at **milestone** level in §8 until prioritised; once scheduled, authors **decompose** into **technical tasks** in `specify/tasks/*.md` **before** merge, with **XIV** layer gates.
- **Story map (`tools/storymap/`)**: Optional **prioritised** backlog (see `tools/storymap/docs/format.md` v2): **exactly one card per `userStoryId`** in the **owning Epic** column (each **`US.x`** is bound to one milestone, so the swimlane is determined by the cited **US**); **`refs`** list **all `T###`** that implement that **US** — **not** a substitute for `spec.md`, `plan.md`, `traceability.md`, or task files.
