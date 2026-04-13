# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-13

### Added
- **Multi-Tenant Identity & RBAC**:
    - Organization registration and Owner setup.
    - User invitation system (Owner, Admin, User roles).
    - Group management (Create groups within Organization).
    - Secure JWT-based Authentication.
- **Security & Safety**:
    - Hardened PostgreSQL **Row Level Security (RLS)** using `FORCE ROW LEVEL SECURITY`.
    - Dedicated restricted `inventiv_app` database user for execution isolation.
- **Infrastructure**:
    - Modular Hexagonal Architecture in Rust.
    - TDD implementation for Domain and Integration layers.
    - Dockerized Postgres (with pgvector) and Redis.
- **Documentation**:
    - Comprehensive README.md and Project Constitution.
    - Synthetic CHANGELOG initialization.
