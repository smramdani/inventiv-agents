# Technical Plan: InventivAgents (v1.0.0 MVP)

## 1. Global Architecture: The Agentic Kernel
The Rust backend act as an orchestration kernel. It manages the multi-tenant lifecycle of **Sessions** and the reasoning loop of **Mission-Driven Agents**.

## 2. Roadmap (v1.0.0)

| Milestone | Focus | Deliverables |
| :--- | :--- | :--- |
| **M1 & M2 (Done)** | **Base Foundation** | Identity, Auth, RLS, Traceability, Telemetry. |
| **M3 (Done)** | **Registry & Entities** | DB schema, domain, `AgentsRepository`, Admin/Owner management API, RLS tests (`specify/tasks/003_milestone_3.md`). |
| **M4a (done — automated gate)** | **LLM + SSE, no tools / no MCP** | OpenAI-compatible client, org-scoped provider resolution, **`POST /org/agents/:id/complete/stream`** (SSE), TraceID; validate with **`specify/mvp-engine-validation.md`** (manual SSE + sign-off still recommended). Phases **1–3** in `004_milestone_4.md`. |
| **M4b (current)** | **Tools + MCP + persistence** | **Phase 4**: HTTP JSON-RPC MCP client (`src/infrastructure/mcp/`). **Phases 5–6**: run/metrics persistence (RLS), full reasoning orchestration + API — **`specify/tasks/004_milestone_4.md`**. |
| **M5** | **The Sovereign Cockpit** | Secure Chat Sessions + RLS session sharing + Audit/Cost dashboard. |

## 3. Component Design

### LLM Abstraction Layer (M4a — in progress / validate)
- A generic service to talk to OpenAI-compatible APIs (OpenRouter, Azure, OVH).
- Standardized streaming (SSE) for real-time interaction.

### MCP Client Implementation (M4b — Phase 4 in progress)
- JSON-RPC over HTTP(S) toward MCP servers registered as skills (`McpHttpJsonRpcClient`); stdio transport remains future work.
- Tool discovery via `tools/list`; orchestration into the live agent stream is **Phase 6**.

### Reasoning Loop Logic (M4a: domain model only; M4b: full orchestration)
- **M4a**: Single-turn LLM completion over SSE; token usage in SSE `usage` event (no tool branches executed).
- **M4b**: `Reasoning` → `Tool Selection` → `Execution` → `Validation` → `Response` with MCP; built-in cost tracking persisted per step where required.

## 4. Security & Safety
- **Isolation**: Row Level Security (RLS) for every entity (Skills, Agents, Sessions).
- **Sandboxing (M4b+)**: MCP tools are strictly restricted by the Agent Template definition once tool execution ships.
