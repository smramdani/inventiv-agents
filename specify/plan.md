# Technical Plan: InventivAgents (v1.0.0 MVP)

## 1. Global Architecture: The Agentic Kernel
The Rust backend act as an orchestration kernel. It manages the multi-tenant lifecycle of **Sessions** and the reasoning loop of **Mission-Driven Agents**.

## 2. Roadmap (v1.0.0)

| Milestone | Focus | Deliverables |
| :--- | :--- | :--- |
| **M1 & M2 (Done)** | **Base Foundation** | Identity, Auth, RLS, Traceability, Telemetry. |
| **M3 (Done)** | **Registry & Entities** | DB schema, domain, `AgentsRepository`, Admin/Owner management API, RLS tests (`specify/tasks/003_milestone_3.md`). |
| **M4** | **The Agentic Engine** | Sovereign API abstraction (SSE) + MCP Client + Reasoning Loop logic. |
| **M5** | **The Sovereign Cockpit** | Secure Chat Sessions + RLS session sharing + Audit/Cost dashboard. |

## 3. Component Design

### LLM Abstraction Layer
- A generic service to talk to OpenAI-compatible APIs (OpenRouter, Azure, OVH).
- Standardized streaming (SSE) for real-time interaction.

### MCP Client Implementation
- JSON-RPC over SSE/Stdio.
- Automatic discovery of tools from connected MCP servers.

### Reasoning Loop Logic
- `Reasoning` -> `Tool Selection` -> `Execution` -> `Validation` -> `Response`.
- Built-in cost tracking (tokens in/out) per reasoning step.

## 4. Security & Safety
- **Isolation**: Row Level Security (RLS) for every entity (Skills, Agents, Sessions).
- **Sandboxing**: MCP tools are strictly restricted by the Agent Template definition.
