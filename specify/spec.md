# Specification: InventivAgents Platform

## 1. Product Vision
An open-source B2B agentic AI platform (AGPL-3.0) enabling SMEs to create, consume, and share AI models and "Skills" within a secure, collaborative, and financially controlled environment.

## 2. Organization Structure & Users
### Roles (RBAC)
- **Owner**: Total control over the organization, payments, and access.
- **Admin**: Manages users, groups, subscriptions, and creates models/skills.
- **User**: Creates and uses AI sessions based on authorized resources.

### Group Management
- A user can belong to 0 or N groups.
- Groups are strictly within an Organization (Teams, Departments, Projects).
- Roles within groups: **Members** or **Organizers**.

## 3. LLM Model Management & Marketplace
- **Consumption**: Subscribe to third-party models (Eval, Free, by Seat, by Tokens).
- **Personal Contribution**: Register own models (API URL + Keys).
- **Exposition**: Propose models to other organizations via subscription.

## 4. Skill Management
- **Skill Definition**: Name, Description, System Prompt, Tool List (Std, Connectors, MCP), associated LLM model.
- **Attribution**: Available Skills per user or per group.
- **Sharing**: Marketplace for Skills between organizations.

## 5. Agentic Sessions & Collaboration
- **Configuration**: Name, Description, choice of Model and active Skills.
- **RAG/Context**: Add documents to the session.
- **Actions**: Share with group/org, archive, delete.

## 6. "Coding" Capabilities (Secure Environment)
- **Full Lifecycle**: Create, modify, version (Git), build, test, and deploy.
- **Sandboxing**: Execution via secure virtualized containers.

## 7. Internationalization (i18n)
- **Default Locales**: English (`en_US`), French (`fr_FR`), Arabic (`ar_AR`).
- **Hierarchy**: Organization has a preferred locale; Users can override it with their own preference.
- **Full Support**: All frontend texts, error messages, and agent system prompts must be translatable.

## 8. User Stories

### For the Owner/Admin
- **US.1**: As an Admin, I want to invite users to my Organization so they can access AI resources.
- **US.2**: As an Admin, I want to create a Group and assign specific Skills to it so that only relevant team members can use them.
- **US.3**: As an Owner, I want to link a payment method and monitor token usage per user to control costs.
- **US.4**: As an Admin, I want to create a custom Skill using an MCP server to integrate our internal tools with the AI.

### For the User
- **US.5**: As a User, I want to login via SSO (Google/GitHub) for a seamless experience.
- **US.6**: As a User, I want to create a Session and select specific Skills to help me with a coding task.
- **US.7**: As a User, I want to share my agent session with my Group members to collaborate on a complex problem.

### For the Developer (Observability)
- **US.8**: As a Developer, I want to trace a single request from the Frontend to the Backend using a TraceID to debug production issues efficiently.

## 9. Open Source & License
- **License**: AGPL-3.0.
- **Language**: All artifacts (Code, Docs, Spec) must be in English.
