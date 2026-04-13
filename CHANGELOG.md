# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-13

### Added
- Initial project structure following Hexagonal Architecture.
- Spec Kit integration and customized Constitution for Rust.
- Core Domain models for Identity (Organization, User, Roles).
- Multi-tenancy support using PostgreSQL Row Level Security (RLS) with dedicated `inventiv_app` role.
- Initial API skeleton with Organization registration.
- Observability schema (Audit logs, Telemetry, Metrics).
- Internationalization support (EN, FR, AR).
- Docker infrastructure (PostgreSQL with pgvector, Redis).
- Connection to GitHub repository `smramdani/inventiv-agents`.
