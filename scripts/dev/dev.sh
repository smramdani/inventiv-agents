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
  up         Start Postgres + Redis via Compose (requires Docker)
  down       Stop Compose stack (no-op if Docker is unavailable)
  migrate    Apply SQL migrations (fails if DB unreachable; exits 2 if schema exists — use reset)
  reset      docker compose down -v, then up + migrate (wipes local DB volume)
  ready      Ensure Postgres (Compose or host TCP) + migrate_try (schema exists = OK)
  test       up + migrate_try + cargo test (loads .env)
  test-lib   Load .env + cargo test --lib only (no Docker required for unit tests)
  run        up + migrate_try + cargo run (remaining args forwarded like normal cargo run)
  run-rel    up + migrate_try + cargo run --release (args forwarded)
  check      fmt + clippy + full cargo test when Docker or host Postgres is reachable; otherwise fmt + clippy + cargo test --lib
  check-local Load .env + fmt --check + clippy -D warnings + cargo test --lib (never uses Docker)
  full       Full pipeline: up + migrate (strict) + cargo test + cargo build --release
  m4a-smoke  M4a MVP manual gate (curl): register → login → provider+key → agent → SSE (needs API up + M4A_LLM_API_KEY)

Environment:
  DOCKER_COMPOSE   Override Compose invocation (default: "docker compose")

Examples:
  ./scripts/dev/dev.sh ready && ./scripts/dev/dev.sh run
  ./scripts/dev/dev.sh run -- --help
  ./scripts/dev/with-env.sh cargo test --test identity_rls
  M4A_LLM_API_KEY=sk-... ./scripts/dev/dev.sh m4a-smoke
USAGE
}

cmd_doctor() {
  inventiv_ensure_env
  inventiv_load_env
  echo "==> Doctor"
  echo "  Repository: $INVENTIV_ROOT"
  local ok=0

  if command -v docker >/dev/null 2>&1; then
    echo "  Docker: OK ($(docker --version))"
    if $DC version >/dev/null 2>&1; then
      echo "  Compose: OK ($($DC version))"
      if $DC exec -T db pg_isready -h 127.0.0.1 -p 5432 -U "${POSTGRES_USER:-inventiv_user}" -d "${POSTGRES_DB:-inventiv_agents}" >/dev/null 2>&1; then
        echo "  Postgres (Compose db): reachable"
        ok=1
      else
        echo "  Postgres (Compose db): not ready (start with: make up)"
      fi
    else
      echo "  Compose: MISSING or broken ($DC)"
    fi
  else
    echo "  Docker: MISSING from PATH (optional if you use host Postgres + psql)"
  fi

  if [[ "$ok" -eq 0 ]]; then
    echo "  Checking host Postgres ${POSTGRES_HOST:-127.0.0.1}:${POSTGRES_PORT:-5432}..."
    if inventiv_postgres_tcp_ok; then
      echo "  Postgres (TCP): port open"
      if command -v psql >/dev/null 2>&1 && psql_ping_doctor; then
        echo "  psql (superuser / migrate role): login OK"
        ok=1
      elif command -v psql >/dev/null 2>&1; then
        echo "  psql: present but login failed (check POSTGRES_* or MIGRATE_DATABASE_URL)"
      else
        echo "  psql: not on PATH (install libpq for host-only migrations)"
      fi
    else
      echo "  Postgres (TCP): port not reachable"
    fi
  fi

  echo "  .env: OK"
  echo "  DATABASE_URL: ${DATABASE_URL:0:48}..."

  if [[ "$ok" -eq 1 ]]; then
    echo "==> Doctor: database access OK (Compose and/or host)"
    return 0
  fi
  echo "==> Doctor: failed — start Docker stack or local Postgres, then retry" >&2
  return 1
}

psql_ping_doctor() {
  if [[ -n "${MIGRATE_DATABASE_URL:-}" ]]; then
    psql "$MIGRATE_DATABASE_URL" -tAc "SELECT 1" >/dev/null 2>&1
  else
    PGPASSWORD="${POSTGRES_PASSWORD:-inventiv_password}" psql \
      -h "${POSTGRES_HOST:-127.0.0.1}" -p "${POSTGRES_PORT:-5432}" \
      -U "${POSTGRES_USER:-inventiv_user}" -d "${POSTGRES_DB:-inventiv_agents}" \
      -tAc "SELECT 1" >/dev/null 2>&1
  fi
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
      inventiv_ensure_local_database
      inventiv_migrate_try
      ;;
    test)
      inventiv_ensure_local_database
      inventiv_migrate_try
      inventiv_load_env
      cargo test "$@"
      ;;
    test-lib)
      inventiv_load_env
      cargo test --lib "$@"
      ;;
    run)
      inventiv_ensure_local_database
      inventiv_migrate_try
      inventiv_load_env
      cargo run "$@"
      ;;
    run-rel)
      inventiv_ensure_local_database
      inventiv_migrate_try
      inventiv_load_env
      cargo run --release "$@"
      ;;
    check)
      inventiv_load_env
      cargo fmt --all -- --check
      cargo clippy --all-targets -- -D warnings
      if inventiv_has_docker; then
        inventiv_docker_up
        inventiv_migrate_try
        cargo test "$@"
      elif inventiv_postgres_tcp_ok; then
        echo "==> Docker not used; host Postgres reachable — running migrations + full integration tests." >&2
        inventiv_migrate_try
        cargo test "$@"
      else
        echo "==> Docker/Compose not available and Postgres TCP (${POSTGRES_HOST:-127.0.0.1}:${POSTGRES_PORT:-5432}) not reachable; running cargo test --lib only." >&2
        echo "==> Start Docker Desktop, or start local Postgres + psql (see README / .env.example), then re-run for full checks." >&2
        cargo test --lib "$@"
      fi
      ;;
    check-local)
      inventiv_load_env
      cargo fmt --all -- --check
      cargo clippy --all-targets -- -D warnings
      cargo test --lib "$@"
      ;;
    full)
      bash "$INVENTIV_ROOT/scripts/dev/test-local-full.sh"
      ;;
    m4a-smoke)
      inventiv_load_env
      bash "$INVENTIV_ROOT/scripts/dev/m4a-mvp-smoke.sh" "$@"
      ;;
    *)
      echo "Unknown command: $cmd" >&2
      usage >&2
      exit 1
      ;;
  esac
}

main "$@"
