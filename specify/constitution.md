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

## Tech Stack
- **Language**: Rust (Stable)
- **Package Manager**: `cargo`
- **Async Runtime**: `tokio`
- **Serialization**: `serde`
- **Database**: PostgreSQL with `sqlx`

## Development Workflow
1. **Specify**: Define requirements in `specify/spec.md`.
2. **Plan**: Define architecture in `specify/plan.md`.
3. **Implement**: Code in short, testable iterations, validated by `cargo check` and `cargo test`.

**Version**: 1.2.0 | **Ratified**: 2026-04-09 | **Changes**: Switched to English and AGPL-3.0 License.
