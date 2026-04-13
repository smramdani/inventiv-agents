# Milestone 3: Registry & Entities (Providers, Skills, Agents)

## Tasks

### T3.1: Database Schema Migration
- [ ] Create `003_agents_registry.sql` migration.
- [ ] Add `llm_providers` table (ID, URL, encrypted API Key).
- [ ] Add `skills` table (ID, type: MCP/Native, endpoint/code, configuration).
- [ ] Add `agents` table (ID, Mission, Persona, LLM choice).
- [ ] Add `agent_skills` junction table.
- [ ] Enable **RLS** and **FORCE RLS** on all new tables.

### T3.2: Domain Models Implementation
- [ ] Create `src/domain/agents/provider.rs`.
- [ ] Create `src/domain/agents/skill.rs`.
- [ ] Create `src/domain/agents/agent.rs`.
- [ ] Write TDD unit tests for each model (validation of URLs, missions, etc.).

### T3.3: Repository Adapters
- [ ] Implement `AgentsRepository` in `src/infrastructure/database/agents.rs`.
- [ ] Ensure `set_rls_context` is used for all operations.

### T3.4: Management API Handlers
- [ ] Implement handlers for registering Providers, Skills, and Agents.
- [ ] Protected by Admin/Owner role.

## Validation
- [ ] Integration tests verify that an Admin can create an Agent with multiple Skills.
- [ ] RLS check: Org A cannot see Org B's Agents.
