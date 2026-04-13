# InventivAgents Constitution (Rust)

## Core Principles

### I. English-First (Open Source Standard)
All documentation, specifications, code comments, variable names, and commit messages must be in English. This ensures maximum accessibility for the international open-source community.

### II. Licensing (AGPL-3.0)
The project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0). All contributions must respect this license. Any derivative work or hosted version must make its source code available.

### III. Idiomatic Rust
Code must follow standard Rust conventions. Strict use of `cargo fmt` and `cargo clippy` is required. We adhere to the "Rust API Guidelines".

### IV. Memory Safety & Security
Maximum exploitation of the Ownership and Borrowing system to guarantee memory safety. `unsafe` code is prohibited unless strictly justified and isolated.

### V. Integrated Testing (Non-Negotiable)
All business logic must have unit tests within the same file (`mod tests`) and integration tests in `tests/`. We prioritize `cargo test`.

### VI. Error Handling
Systematic use of the `Result<T, E>` type. No "panics" in production code (no `unwrap()` or `expect()` except in tests or proven impossible cases). Use `anyhow` or `thiserror` for clean error management.

### VII. Clean Engineering & Code Clarity
- **Clean Code Philosophy**: We prioritize readability over cleverness. Code must be self-explanatory.
- **Single Responsibility Principle (SRP)**: Each module, trait, or struct must have one clear responsibility.
- **One File, One Scope**: Files must remain small and focused. If a file exceeds ~300 lines or covers multiple domains, it must be refactored into a sub-module.
- **Separation of Concerns**: Infrastructure (DB, API) must be strictly decoupled from Domain Logic (Agents, Auth) using the Hexagonal Architecture pattern.
- **Explicit over Implicit**: No "magic" macros or complex abstractions that hide intent.

### VIII. Test-Driven Development (TDD) Strategy
- **100% Coverage Target**: All business logic must have 100% unit test coverage before being merged.
- **TDD Cycle**: Red (Write failing test) -> Green (Implement logic) -> Refactor (Optimize and clean).
- **Comprehensive Testing**: 
    - **Unit Tests**: For individual functions and business rules.
    - **Integration Tests**: For database interactions and cross-module logic.
    - **API Tests**: Exhaustive testing of endpoints for both successful (Passing) and failed (Non-passing/Edge cases) scenarios.
- **Verification**: No feature is considered "Done" until all tests pass and are integrated into the CI pipeline.

### IX. Systematic Observability & Traceability
- **Structured Logging**: All backend and frontend actions must be logged using structured formats (JSON) with appropriate levels: `DEBUG`, `INFO`, `WARN`, `ERROR`.
- **Full Traceability**: Every request must carry a unique `TraceID` across the entire stack (FE -> API -> DB -> Agents) to allow correlation of logs.
- **Frontend Telemetry**: Frontend exceptions and significant traces must be systematically sent to the backend and persisted for audit and debugging.
- **Exception Catching**: No exception should be swallowed silently. Every error must be caught, logged with context, and reported appropriately.
- **Phase-Agnostic**: Observability is mandatory in Dev, Staging, and Production, as well as during Build and Support phases.

### X. Versioning, Lifecycle & Change Management
- **Systematic & Legible Commits**: All project changes must be committed to GitHub systematically. Commit messages must be clear, following the Conventional Commits standard (e.g., `feat:`, `fix:`, `docs:`).
- **Semantic Versioning (SemVer)**: Releases must be versioned using Semantic Versioning (Major.Minor.Patch).
- **Version Visibility**: The current version of both back-end and front-end must be explicitly displayed in the logs at startup and within the user interface. This is mandatory for debugging and support.
- **Synthetic Changelog**: A clear and synthetic `CHANGELOG.md` must be maintained, summarizing all significant changes per version.

### XI. Project Documentation
- **Mandatory README.md**: A clear, helpful, and accurate `README.md` must be maintained in the root directory. It must be kept up to date with every major change.
- **README Content**: It must provide:
    - **Purpose**: A high-level explanation of the project's goals and vision.
    - **Features**: A comprehensive list of current and planned capabilities.
    - **Installation**: Step-by-step instructions (Prerequisites, Setup).
    - **Usage**: Practical examples of how to use the API or CLI.
    - **Contribution**: Clear guidelines on how community members can help.

### XII. Spec Kit & Spec-Driven Development (SDD)
- **SDD Lifecycle**: No code implementation shall begin without a completed Spec (Functional), Technical Plan (Architecture), and Task List (Execution) generated via Spec Kit.
- **Sync Requirement**: Artifacts must be kept in sync with the code. If the implementation deviates from the plan, the plan must be updated and ratified first.
- **Traceability**: Every task in the task list must link back to a User Story or a specific technical requirement in the Spec.
- **Layered Definition of Done**: Before a feature or task group is marked complete, the criteria in **XIV** must be satisfied for every application layer the change touches. Use `/speckit.checklist` (or an equivalent review) to record layer gates when automation is not yet available.

### XIII. Agentic Hierarchy & Logic
- **Skill**: The atomic unit of capability. Powered by MCP servers or native Rust functions.
- **Agent**: A mission-oriented entity. It is defined by a **Mission**, a **Persona**, and a **Toolbelt** (a selection of 0..N Skills).
- **Session**: The secure, multi-tenant execution context where a User interacts with an Agent or uses Skills directly.
- **Reasoning Loop**: Agents must follow a structured loop (Reasoning -> Tool Selection -> Execution -> Validation -> Response).

### XIV. Definition of Done by Layer
Delivery must combine a **vertical slice owner** (end-to-end coherence for the user story or spec requirement) with explicit **layer gates** below. If a layer is not in scope, state **N/A** with a one-line rationale in the task notes or pull request.

#### Vertical slice (always required)
- **Story linkage**: Implementation maps to a User Story id or a numbered requirement in `specify/spec.md`; tasks cite that reference.
- **Independent value**: The slice can be tested and demonstrated without relying on unfinished sibling stories, except where the plan documents a hard dependency.
- **Artifact alignment**: `specify/spec.md`, `specify/plan.md`, and the active task list reflect behavior shipped; any intentional deviation is updated in spec/plan first, per XII.

#### Database
- **Migrations**: Schema changes use reviewed SQL migrations with clear upgrade semantics; risky changes note rollback or repair strategy.
- **Tenancy and RLS**: Multi-tenant tables have policies (including `FORCE RLS` where required) matching the access model in the plan; constraints (FK, uniqueness, check) match domain rules.

#### Backend — domain and application core
- **Domain purity**: Business rules live in the domain/application layers as per hexagonal boundaries (VII), not leaked into handlers or repositories beyond mapping.
- **Tests**: New or changed domain logic includes `mod tests` coverage; integration tests cover new DB or cross-module behavior where specified (V, VIII).
- **Errors**: Production paths avoid `unwrap()`/`expect()` except as allowed in VI.

#### API and HTTP surface
- **Authn/Authz**: Routes enforce authentication and role or scope checks consistent with the spec and RLS assumptions.
- **Contracts**: Request/response shapes and status codes match documented API contracts when they exist; breaking changes are versioned or coordinated per X.
- **Observability**: Structured logging and request correlation (**TraceID** or equivalent) on success and failure paths (IX).

#### Front-end and clients (when applicable)
- **UX states**: Loading, empty, success, and error states are defined and implemented; errors are safe for end users (no secrets, no raw stack traces).
- **Telemetry**: Significant failures and actions are observable per IX when the product includes a client for this feature.

#### Cross-cutting
- **Security**: New surfaces receive an explicit security pass (input validation, authz gaps, rate limits or abuse considerations when relevant).
- **Documentation**: User-visible or operator-facing behavior updates `README.md` and `CHANGELOG.md` per X and XI.
- **Quality bar**: `cargo fmt`, `cargo clippy`, and `cargo test` pass for affected crates before merge (III, VIII).

## Tech Stack
- **Language**: Rust (Stable)
- **Package Manager**: `cargo`
- **Async Runtime**: `tokio`
- **Serialization**: `serde`
- **Database**: PostgreSQL with `sqlx`

## Development Workflow
1. **Specify**: Define requirements in `specify/spec.md`.
2. **Plan**: Define architecture in `specify/plan.md`.
3. **Tasks**: Break down execution in the feature task list (Spec Kit `/speckit.tasks` or `specify/tasks/`), with each task traceable to a user story or spec clause (XII). Where useful, tag tasks with the layers touched (for example DB, API, FE) to make XIV review explicit.
4. **Checklist / Analyze** (recommended before implementation): Run `/speckit.checklist` and/or `/speckit.analyze` to validate spec, plan, tasks, and **XIV layer gates** for the slice.
5. **Implement**: Short, testable iterations; `cargo check` and `cargo test` on affected code.
6. **Done**: Confirm XIV for every touched layer; update spec/plan/tasks if reality justified a change (XII).

**Version**: 1.3.0 | **Ratified**: 2026-04-09 | **Last Amended**: 2026-04-13 | **Changes**: Added XIV Definition of Done by Layer; extended XII and Development Workflow for Spec Kit quality gates.

**Spec Kit**: The ratified constitution lives in `specify/constitution.md` and is **mirrored** at `.specify/memory/constitution.md` for Spec Kit (`/speckit.plan`, `/speckit.analyze`, etc.). The two files must stay identical; amend both on every constitution change.
