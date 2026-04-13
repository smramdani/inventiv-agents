#!/usr/bin/env bash
# Unified local workflow: Docker stack, migrations, tests, API.
# Run from anywhere:  ./scripts/dev/dev.sh <command>
# Or from repo root:   make <target>

set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"
# shellcheck disable=SC1091
source "$ROOT/scripts/dev/lib.sh"

usage() {
  cat <<'USAGE'
InventivAgents local dev tool

Usage:
  ./scripts/dev/dev.sh <command> [args...]

Commands:
  help       Show this message
  doctor     Check Docker, Compose, .env, and Postgres connectivity
  env        Ensure .env exists (copy from .env.example if missing)
  up         Start Postgres + Redis (wait until healthy)
  down       Stop stack (docker compose down)
  migrate    Apply SQL migrations (fails if DB unreachable; exits 2 if schema exists — use reset)
  reset      docker compose down -v, then up + migrate (wipes local DB volume)
  ready      up + migrate (treat “schema exists” as OK)
  test       up + migrate_try + cargo test (loads .env)
  test-lib   Load .env + cargo test --lib only (no Docker required for unit tests)
  run        up + migrate_try + cargo run (remaining args forwarded like normal cargo run)
  run-rel    up + migrate_try + cargo run --release (args forwarded)
  check      up + migrate_try + cargo fmt --check + clippy -D warnings + test
  full       Full pipeline: up + migrate (strict) + cargo test + cargo build --release

Environment:
  DOCKER_COMPOSE   Override Compose invocation (default: "docker compose")

Examples:
  ./scripts/dev/dev.sh ready && ./scripts/dev/dev.sh run
  ./scripts/dev/dev.sh run -- --help
  ./scripts/dev/with-env.sh cargo test --test identity_rls
USAGE
}

cmd_doctor() {
  inventiv_ensure_env
  inventiv_load_env
  echo "==> Doctor"
  echo "  Repository: $INVENTIV_ROOT"
  if command -v docker >/dev/null 2>&1; then
    echo "  Docker: OK ($(docker --version))"
  else
    echo "  Docker: MISSING from PATH"
    return 1
  fi
  if $DC version >/dev/null 2>&1; then
    echo "  Compose: OK ($($DC version))"
  else
    echo "  Compose: MISSING ($DC)"
    return 1
  fi
  echo "  .env: OK"
  echo "  DATABASE_URL: ${DATABASE_URL:0:48}..."
  inventiv_docker_up >/dev/null
  if $DC exec -T db pg_isready -h 127.0.0.1 -p 5432 -U "${POSTGRES_USER:-inventiv_user}" -d "${POSTGRES_DB:-inventiv_agents}" >/dev/null 2>&1; then
    echo "  Postgres: reachable"
  else
    echo "  Postgres: NOT reachable"
    return 1
  fi
  echo "==> All checks passed"
}

cmd_migrate_strict() {
  bash "$INVENTIV_ROOT/scripts/db/apply-migrations.sh"
}

main() {
  local cmd="${1:-help}"
  shift || true

  case "$cmd" in
    help | -h | --help)
      usage
      ;;
    doctor)
      cmd_doctor
      ;;
    env)
      inventiv_ensure_env
      echo ".env ready at $INVENTIV_ROOT/.env"
      ;;
    up)
      inventiv_docker_up
      ;;
    down)
      inventiv_docker_down "$@"
      ;;
    migrate)
      cmd_migrate_strict
      ;;
    reset)
      bash "$INVENTIV_ROOT/scripts/db/reset-local-db.sh"
      ;;
    ready)
      inventiv_docker_up
      inventiv_migrate_try
      ;;
    test)
      inventiv_docker_up
      inventiv_migrate_try
      inventiv_load_env
      cargo test "$@"
      ;;
    test-lib)
      inventiv_load_env
      cargo test --lib "$@"
      ;;
    run)
      inventiv_docker_up
      inventiv_migrate_try
      inventiv_load_env
      cargo run "$@"
      ;;
    run-rel)
      inventiv_docker_up
      inventiv_migrate_try
      inventiv_load_env
      cargo run --release "$@"
      ;;
    check)
      inventiv_docker_up
      inventiv_migrate_try
      inventiv_load_env
      cargo fmt --all -- --check
      cargo clippy --all-targets -- -D warnings
      cargo test "$@"
      ;;
    full)
      bash "$INVENTIV_ROOT/scripts/dev/test-local-full.sh"
      ;;
    *)
      echo "Unknown command: $cmd" >&2
      usage >&2
      exit 1
      ;;
  esac
}

main "$@"
