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
- **Usage Tracking**: Systematically log token consumption in the `metrics` table for cost analysis.

## 4. Skills, Agents & MCP (v1.0.0)
### MCP Skills
- **Definition**: Atomic unit of capability connected via Model Context Protocol.
- **Types**: Internal Knowledge Base (PDF Search), SQL connectors, or custom external MCP servers.

### Mission-Driven Agents
- **Definition**: Specialized AI personas with a defined **Mission**, **Persona**, and a **Toolbelt** (0..N Skills).
- **Control**: Admins decide which Agents have access to which Skills.

## 5. Agentic Sessions & Collaboration (v1.0.0)
- **Execution Space**: Multi-tenant context where reasoning loops occur.
- **Context**: Document uploads (RAG) and message history.
- **Sharing**: Sessions can be shared within a Group for collaboration.

## 6. User Stories (MVP)
- **US.1 (Admin)**: I want to register an **OVH Cloud AI** endpoint so our data remains within European sovereignty.
- **US.2 (Admin)**: I want to connect an MCP server to create a "Internal Policy" Skill.
- **US.3 (Admin)**: I want to create an "HR Agent" with the mission to help employees and access to the "Policy" Skill.
- **US.4 (User)**: I want to start a session with the "HR Agent" to ask about my holiday balance.
- **US.5 (Owner)**: I want to see a report of token usage per department to monitor my AI budget.

## 7. Open Source & License
- **License**: AGPL-3.0.
- **Language**: English.
