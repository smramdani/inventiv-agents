# InventivAgents Constitution (Rust)

## Core Principles

### I. English-First (Open Source Standard)
All **repository** documentation, specifications, roadmaps, backlogs, user stories in tooling, `README` / `CHANGELOG`, and contributor-oriented code comments must be in **English**. Variable names and commit messages follow the same rule. **Application localization** (end-user UI copy, locale catalogs, supported account locales such as `fr_FR` / `ar_SA`) is exempt: those artifacts exist solely for product i18n and may use multiple languages by design.

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
- **Traceability**: Every **technical task** must link back to **`specify/plan.md` §2** (milestone / prioritised backlog) and to **exactly one** **User Story** (**`US.1`–`US.15`**, **`specify/spec.md` §7**), with **Epic** (**`spec.md` §2**) on the same row for journey context. Each **`US.x`** is bound to **exactly one milestone** (so citing the **US** also pins the task to its **Epic** and **milestone**). When the **story map** (`tools/storymap/backlog/*.json`) is used, each **`userStoryId`** appears on **exactly one card** (= one card per US per milestone); **`columnId`** is the **owning Epic** column for that **US**; **`refs`** list every **`T###`** that implements that **US**. Data must stay consistent with **`specify/traceability.md`** so **Spec ↔ Traceability ↔ Plan ↔ Tasks ↔ Story map** stay aligned (**XVI**).
- **Layered Definition of Done**: Before a feature or task group is marked complete, the criteria in **XIV** must be satisfied for every application layer the change touches. Use `/speckit.checklist` (or an equivalent review) to record layer gates when automation is not yet available.
- **Release discipline**: Delivery pipelines follow **XV**—one immutable release artifact promoted across environments, with configuration and secrets external to the package.

### XIII. Agentic Hierarchy & Logic
- **Skill**: The atomic unit of capability. Powered by MCP servers or native Rust functions.
- **Agent**: A mission-oriented entity. It is defined by a **Mission**, a **Persona**, and a **Toolbelt** (a selection of 0..N Skills).
- **Session**: The secure, multi-tenant execution context where a User interacts with an Agent or uses Skills directly.
- **Reasoning Loop**: Agents must follow a structured loop (Reasoning -> Tool Selection -> Execution -> Validation -> Response).

### XIV. Definition of Done by Layer
Delivery must combine a **vertical slice owner** (end-to-end coherence for the user story or spec requirement) with explicit **layer gates** below. If a layer is not in scope, state **N/A** with a one-line rationale in the task notes or pull request.

#### Vertical slice (always required)
- **Story linkage**: Implementation maps to **exactly one** **User Story** id (**`US.1`–`US.15`**, **`spec.md` §7**), **`plan.md` §2** milestone row, and the owning **Epic** id (**`spec.md` §2**); tasks cite those references. The cited **US** itself lives in **exactly one milestone**, so the task’s **Epic** + **Milestone** are determined by the **US** it cites.
- **Independent value**: The slice can be tested and demonstrated without relying on unfinished sibling stories, except where the plan documents a hard dependency.
- **Artifact alignment**: `specify/spec.md`, `specify/plan.md`, and the active task list reflect behavior shipped; when **`tools/storymap/backlog/*.json`** is maintained, it reflects the **same** prioritised **user stories** (**one card per `userStoryId`**, **`refs`** = **`T###`**) (**XVI**). Any intentional deviation is updated in spec/plan first, per XII.

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
- **Release and CD** (when applicable): Changes to build, packaging, or deployment satisfy **XV**—no environment-specific compiled behavior; one artifact promoted with external configuration and secrets.

### XV. Immutable Release Artifact, Promotion, and Externalized Configuration (CI/CD)
These rules align delivery with **build once, promote many** and **twelve-factor** style configuration. They apply to every deployable component (API binary, container image, or future front-end bundle).

#### Immutable artifact and promotion
- **Single package per release**: For a given version or release identifier (for example SemVer tag or digest), **one** build produces **one** deployable artifact that is **identical** across local integration, staging, and production. Environments differ only by **what is attached at runtime**, not by recompiling or rebuilding a separate artifact per environment for the same release.
- **Promotion, not re-build**: Progression is **dev → staging → production** by **promoting the same validated artifact** (or the same immutable image digest) along controlled stages. Downstream stages may add gates (smoke tests, approvals); they must not introduce ad hoc rebuilds that change the bits under the same release label.
- **Traceability**: The artifact carries an explicit **version or digest** visible at runtime (IX, X) and in deployment records so operators can prove what is running in each environment.

#### Configuration and behavior outside the package
- **No environment logic in source**: Application source must not encode environment-specific behavior (for example `if env == "prod"`) for **deployment** concerns. Feature flags that are **data-driven** and loaded from configuration or the database are acceptable when they are not used to bypass quality or security gates.
- **External configuration**: All deployment-specific settings—URLs, pool sizes, feature toggles, log levels, third-party endpoints—are supplied via **environment variables**, **mounted configuration files**, or **platform configuration services**, appropriate to the host (local Compose, Kubernetes, Scaleway Serverless Containers, etc.).
- **Secrets**: Secrets are **never** embedded in images or binaries. They are injected at runtime from a **secret manager** or equivalent mechanism.
- **Data over code for tenant behavior**: Organization-specific or tenant-specific behavior that legitimately differs between installations is modeled in the **database** or **external policy stores**, not in private forks of the application package.

#### Pipeline obligations
- **CI** validates the same artifact shape used in production (tests, lint, SCA as adopted) and publishes a **versioned, immutable** output to a trusted registry or artifact store.
- **CD** jobs perform **deploy and configure** only: roll out the pinned artifact, apply environment-specific **config** and **secrets**, run migrations when required, and verify health. **No drift** between “what was tested in staging” and “what runs in prod” except for explicitly externalized configuration and data.

### XVI. Specification depth, epics, milestones, user stories, technical tasks, story map
- **Top-down refinement (strategic → particular)**: Intent flows **down**: **`specify/spec.md`** (**§1** vision, **§2 Epics**, **§3** roles & org, **§§4–6** domain capabilities, **§7 User Stories** — **`US.1`–`US.15`**, each epic maps to one or more stories and each **US** belongs to **exactly one milestone**, **§8** milestones) → **`specify/plan.md`** (**§2** prioritised user stories **per milestone**, plus architecture §§1,3–4) → **`specify/traceability.md`** (**US ↔ `T###` ↔ milestone** matrix) → **`specify/tasks/*.md`** (**elementary technical tasks**—implementation, architecture, verification—for that milestone) → **code**. Lower layers must remain **traceable upward**; no orphan tasks without a **`plan.md` §2** milestone slice and **`spec.md`** anchor (**Epic** + **exactly one `US.x`** per task row).
- **User story ids vs technical task ids**: **`US.1`–`US.15`** (**`spec.md` §7**) denote **product value** for personas and journeys; **each `US.x` belongs to exactly one milestone** — when a theme spans milestones, it is **split into separate `US.x`** (e.g. `US.7` cockpit chat in **M5a** vs `US.11` persisted sessions in **M5b** vs `US.12` tool-augmented chat in **M4b**). **`T###`** (**`specify/tasks/*.md`**) denotes **implementation** work (DB, API, domain, FE, tests, docs); each **`T###`** therefore belongs to exactly one **US**, one **Epic**, and one **milestone**. **Each `T###` row cites exactly one `US.x`**; **several `T###` rows** may **implement the same `US.x`**. Do not treat **`T###`** as a synonym for a user story, do not duplicate a **`US.x`** across milestones, and do not invent informal **`US`** ids — add a proper **`US.x`** in **`spec.md` §7** (new milestone slice) when the product gains a new outcome, then new **`T###`** rows.
- **Bottom-up time horizon**: **Shipped or in-flight** work must be expressed at **testable** granularity—**technical tasks** with acceptance criteria and XIV layer gates—**before** merge. **Future milestones** may stay at **epic / milestone** level in **spec §8** and **plan §2** until **prioritised**; once scheduled, authors **must decompose** into **tasks** and, where the story map is used, into **one card per `userStoryId`** with **`refs`** to **`T###`** **before** coding starts (extends XII).
- **Story map cards**: **One card = one `US.x`** (= one US per milestone, since each US is bound to a single milestone); **`refs`** carry **all `T###`** for that **US**. **Task file lines** (**`T###`** — globally unique, see `specify/plan.md` §2.0) remain **PR-sized** implementation items; they do **not** each get their own story-map card. The viewer rejects backlog files where the same **`userStoryId`** appears on more than one card or on more than one milestone. See `tools/storymap/docs/format.md`.
- **Story map role**: `tools/storymap/` (JSON backlog + viewer) is **optional but recommended** for **prioritised**, **visual** trace of user stories alongside Spec Kit. It does **not** replace `spec.md`, `plan.md`, or task files; it **mirrors** them for granularity and communication.
- **Single delivery window**: When scope or acceptance changes, update **`spec.md` / `plan.md` / tasks`** and, if present, **`tools/storymap/backlog/*.json`** in the **same** change set or PR so the three layers do not diverge.

## Tech Stack
- **Language**: Rust (Stable)
- **Package Manager**: `cargo`
- **Async Runtime**: `tokio`
- **Serialization**: `serde`
- **Database**: PostgreSQL with `sqlx`

## Development Workflow
1. **Specify**: Define requirements in `specify/spec.md` (**§2 Epics**, **§7 User Stories**, domain sections).
2. **Plan**: Define architecture and **§2 milestone prioritisation** (user stories per milestone) in `specify/plan.md`; maintain **`specify/traceability.md`** when **US** membership or **`T###`** sets change.
3. **Tasks**: Break down execution into **elementary technical tasks** in the feature task list (Spec Kit `/speckit.tasks` or `specify/tasks/`), each traceable to **`plan.md` §2** and **exactly one** **User Story** (**`US.1`–`US.15`**, **`spec.md` §7**) plus **Epic** (XII). Tag layers (DB, API, FE) for XIV. **Optional**: **`tools/storymap/backlog/*.json`** with **one card per `userStoryId`** and **`refs`** listing **`T###`** (**XVI**); run **`npm run check:english`** under `tools/storymap/web/` when that JSON changes.
4. **Checklist / Analyze** (recommended before implementation): Run `/speckit.checklist` and/or `/speckit.analyze` to validate spec, plan, tasks, and **XIV layer gates** for the slice.
5. **Implement**: Short, testable iterations; `cargo check` and `cargo test` on affected code.
6. **Done**: Confirm XIV for every touched layer; update spec/plan/tasks if reality justified a change (XII); update **`tools/storymap/backlog/*.json`** when that backlog is maintained (**XVI**).
7. **Release (CD)**: Ship only **immutable, promoted artifacts** per XV; inject environment-specific settings and secrets at deploy time, never by rebuilding a differently configured package for the same release identifier.

**Version**: 1.8.0 | **Ratified**: 2026-04-09 | **Last Amended**: 2026-04-22 | **Changes**: **One US ↔ one milestone** — `US.x` ids extended to **`US.1`–`US.15`** (split themes across milestones into separate `US.x`); story map enforces **one card per `userStoryId`** and rejects cross-milestone reuse (**XII** / **XVI**). Prior **1.7.1**: one card per (`milestoneId`, `userStoryId`); `refs` list `T###`. Prior **1.7.0**: `US.1`–`US.10`; each `T###` maps to exactly one `US.x`; story map v2.

**Spec Kit**: The ratified constitution lives in `specify/constitution.md` and is **mirrored** at `.specify/memory/constitution.md` for Spec Kit (`/speckit.plan`, `/speckit.analyze`, etc.). The two files must stay identical; amend both on every constitution change.
