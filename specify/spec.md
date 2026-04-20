# Specification: InventivAgents Platform (v1.0.0 MVP)

## 1. Product Vision
An open-source B2B agentic AI platform (AGPL-3.0) enabling SMEs to deploy safe, secure, and sovereign AI. The platform provides a "Design-to-Cost" model where companies control their own models (Managed Cloud APIs) and audit all usage.

## 2. Organization Structure & Users (Done)
### Roles (RBAC)
- **Owner**: Total control over the organization, providers, and access.
- **Admin**: Manages users, groups, and creates Agent Templates (Skills).
- **User**: Creates and uses AI sessions based on authorized templates.

### Group Management
- Groups serve as the boundary for sharing Agent Templates and Sessions.

## 3. Sovereign Provider Management (v1.0.0)
- **Managed APIs**: Admins register LLM providers (OVH Cloud AI, Scaleway, Azure OpenAI) via API URL and Keys.
- **Usage Tracking (target)**: Systematically log token consumption in the **`metrics`** table for cost analysis (**US.5** at full maturity).
- **Usage in the cockpit (M5a, shipped)**: The web client surfaces the **last `usage` object** from the SSE stream for the current turn (read-only, client-side). This is an **intentional subset** of US.5 until **M4b** persistence and reporting exist; see §7 **M5a / M5b** and `specify/tasks/005_milestone_5.md`.

## 4. Skills, Agents & MCP (v1.0.0)
### MCP Skills
- **Definition**: Atomic unit of capability connected via Model Context Protocol.
- **Types**: Internal Knowledge Base (PDF Search), SQL connectors, or custom external MCP servers.

### Mission-Driven Agents
- **Definition**: Specialized AI personas with a defined **Mission**, **Persona**, and a **Toolbelt** (0..N Skills).
- **Control**: Admins decide which Agents have access to which Skills.

## 5. Agentic Sessions & Collaboration (v1.0.0)

**Target product (this section)**: A durable **Session** is the multi-tenant execution space for an agent conversation: message history, optional document context (RAG), and **sharing within a Group** for collaboration.

**Phased delivery (aligned with milestone 5 tasks)** — per **Constitution XII** (spec/plan/tasks stay in sync):

- **M5a (shipped — “Sovereign Cockpit” v1)**: Authenticated **web client** with registry CRUD (create/list) for providers, skills, and agents, plus **single-turn / ephemeral chat** against the existing M4a endpoint **`POST /org/agents/:id/complete/stream`** (SSE). There is **no persisted `sessions` table**, no server-side message history, and no group-based sharing yet. This satisfies an early **US.4**-style slice (“talk to an agent in the browser”) without claiming full §5 semantics.
- **M5b (planned)**: Persisted **sessions** (schema + RLS under **`backend/migrations/`**), message history, sharing rules per **§2 Groups**, and cockpit UX for session list/resume/share. Tracked as follow-up work in **`specify/tasks/005_milestone_5.md`**.

## 6. User Stories (MVP)
- **US.1 (Admin)**: I want to register an **OVH Cloud AI** endpoint so our data remains within European sovereignty.
- **US.2 (Admin)**: I want to connect an MCP server to create a "Internal Policy" Skill.
- **US.3 (Admin)**: I want to create an "HR Agent" with the mission to help employees and access to the "Policy" Skill.
- **US.4 (User)**: I want to start a session with the "HR Agent" to ask about my holiday balance.
- **US.5 (Owner)**: I want to see a report of token usage per department to monitor my AI budget.

## 7. Milestones M4 / M5 / M4b (technical path, v1.0.0)

**M4a (shipped)**: Headless **agentic engine** without tool execution in the product path: OpenAI-compatible **LLM**, org-scoped **provider resolution**, **SSE** completion, **TraceID** / structured logging. Minimal **US.4** slice for API clients.

**M5 (current priority)**, split per §5:

- **M5a (shipped)**: **Sovereign Cockpit v1** — `frontend/` Vite/React SPA: **register / login**, **registry** (create + list providers, skills, agents; Owner/Admin), **ephemeral SSE chat** + **last-turn usage** display, **CORS** and **`whoami` role** for RBAC in the UI. Does **not** include persisted sessions, group sharing, or full **US.5** reporting. Tasks and traceability: **`specify/tasks/005_milestone_5.md`** (T5.1–T5.6 for M5a scope).
- **M5b (next within M5)**: Persisted **sessions**, history, and **collaboration** as in §5; extends the same milestone file with new tasks before marking “M5 complete” against §5–6.

**M4b (after M5)**: **US.2** (MCP skills **in the reasoning loop**), full **US.3**-style toolbelt orchestration, **persistence of runs/metrics** (RLS) — `specify/tasks/004_milestone_4.md` Phases **4–6**. An HTTP MCP **client library** may exist earlier as a non-product-critical foundation; **wiring** into live agent streams is **post-M5**.

**M4a validation checklist**: `specify/mvp-engine-validation.md`.

## 8. Open Source & License
- **License**: AGPL-3.0.
- **Language**: English.
