# Technical Plan: InventivAgents (v1.0.0 MVP)

## 1. Global Architecture: The Agentic Kernel
The Rust backend acts as an orchestration kernel. It manages the multi-tenant lifecycle of **Sessions** (persisted in **M5b**; **M5a** cockpit uses ephemeral SSE turns only — see `specify/spec.md` §5–7) and the reasoning loop of **Mission-Driven Agents**.

## 2. Roadmap (v1.0.0)

| Milestone | Focus | Deliverables |
| :--- | :--- | :--- |
| **M1 & M2 (Done)** | **Base Foundation** | Identity, Auth, RLS, Traceability, Telemetry. |
| **M3 (Done)** | **Registry & Entities** | DB schema, domain, `AgentsRepository`, Admin/Owner management API, RLS tests (`specify/tasks/003_milestone_3.md`). |
| **M4a (done — automated gate)** | **LLM + SSE, no tools / no MCP** | OpenAI-compatible client, org-scoped provider resolution, **`POST /org/agents/:id/complete/stream`** (SSE), TraceID; validate with **`specify/mvp-engine-validation.md`**. Phases **1–3** in `004_milestone_4.md`. |
| **M5 (current)** | **The Sovereign Cockpit (front-first)** | **M5a (done):** auth, registry create/list, **ephemeral** SSE chat + last-turn **usage** in `frontend/` (see `specify/spec.md` §7). **M5b (next):** persisted **sessions**, history, group sharing, full **US.5**-style reporting when metrics exist. **Tasks:** `specify/tasks/005_milestone_5.md`. |
| **M4b (after M5)** | **Tools + MCP loop + persisted metrics** | **US.2** (MCP in reasoning), **US.3**-style toolbelt in loop, **Phases 5–6** (runs/metrics RLS, orchestration). HTTP MCP client in `backend/src/infrastructure/mcp/` is **foundation only** until this milestone. **`specify/tasks/004_milestone_4.md`** Phases 4–6. |

## 3. Component Design

### LLM Abstraction Layer (M4a — shipped)
- OpenAI-compatible APIs; SSE streaming for the cockpit and API clients.

### MCP & tool loop (M4b — **after M5**)
- Library: `McpHttpJsonRpcClient` in `backend/src/infrastructure/mcp/` (`tools/list`, `tools/call`) exists; **product integration** (reasoning loop, live SSE + tools) waits until **post-M5** per roadmap.
- Full loop + persisted metrics: **`004_milestone_4.md`** Phases 5–6.

### Reasoning Loop Logic
- **M4a (shipped)**: Single-turn LLM over SSE; token counts in SSE `usage`.
- **M4b (post-M5)**: `Reasoning` → tool selection → MCP execution → validation → response; cost tracking persisted per step where required.

## 4. Security & Safety
- **Isolation**: Row Level Security (RLS) for every entity (Skills, Agents, Sessions).
- **Sandboxing (M4b+)**: MCP tools restricted by agent/skill policy once the tool loop ships (**after M5**).
