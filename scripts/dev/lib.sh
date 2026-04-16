#!/usr/bin/env bash
# Shared helpers for local dev scripts. Source from other scripts in this directory:
#   source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

_INVENTIV_DEV_LIB_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INVENTIV_ROOT="$(cd "${_INVENTIV_DEV_LIB_DIR}/../.." && pwd)"
export INVENTIV_ROOT

# Docker Desktop (macOS) installs the CLI under /Applications/.../bin — often missing from
# non-login shells (Cursor tasks, `make`, CI agents). Prepend known locations before checks.
inventiv_bootstrap_docker_path() {
  if command -v docker >/dev/null 2>&1; then
    return 0
  fi
  if [[ -n "${INVENTIV_DOCKER_BIN:-}" && -d "${INVENTIV_DOCKER_BIN}" && -x "${INVENTIV_DOCKER_BIN}/docker" ]]; then
    export PATH="${INVENTIV_DOCKER_BIN}:$PATH"
    return 0
  fi
  local uname_s
  uname_s="$(uname -s 2>/dev/null || true)"
  case "$uname_s" in
    Darwin)
      local bins=(
        "/Applications/Docker.app/Contents/Resources/bin"
        "/usr/local/bin"
        "/opt/homebrew/bin"
      )
      for d in "${bins[@]}"; do
        if [[ -x "$d/docker" ]]; then
          export PATH="$d:$PATH"
          return 0
        fi
      done
      ;;
    Linux)
      for d in /usr/local/bin /usr/bin; do
        if [[ -x "$d/docker" ]]; then
          export PATH="$d:$PATH"
          return 0
        fi
      done
      ;;
  esac
  return 1
}

export DC="${DOCKER_COMPOSE:-docker compose}"

inventiv_bootstrap_docker_path || true

# TCP check to Postgres (uses POSTGRES_HOST / POSTGRES_PORT after caller loads .env).
inventiv_postgres_tcp_ok() {
  local host="${POSTGRES_HOST:-127.0.0.1}"
  local port="${POSTGRES_PORT:-5432}"
  python3 -c "import socket,sys; h,p=sys.argv[1],int(sys.argv[2]); s=socket.create_connection((h,p),timeout=3); s.close()" "$host" "$port" >/dev/null 2>&1
}

# Start Compose db+redis when Docker works; otherwise require an open Postgres port on the host.
inventiv_ensure_local_database() {
  inventiv_ensure_env
  inventiv_load_env
  if inventiv_has_docker; then
    inventiv_docker_up
    return 0
  fi
  echo "==> Docker not on PATH; using host Postgres at ${POSTGRES_HOST:-127.0.0.1}:${POSTGRES_PORT:-5432} (see POSTGRES_* / MIGRATE_DATABASE_URL in .env)." >&2
  if inventiv_postgres_tcp_ok; then
    echo "==> Postgres TCP port is reachable" >&2
    return 0
  fi
  echo "Cannot reach Postgres. Install Docker Desktop and retry, or start a local Postgres and align POSTGRES_HOST/POSTGRES_PORT with .env." >&2
  exit 1
}

inventiv_ensure_env() {
  cd "$INVENTIV_ROOT" || exit 1
  if [[ ! -f .env ]]; then
    cp .env.example .env
    echo "Created .env from .env.example" >&2
  fi
}

# Load .env into the current shell (caller must not use set -u before sourcing .env with empty vars).
inventiv_load_env() {
  inventiv_ensure_env
  set -a
  # shellcheck disable=SC1091
  source "$INVENTIV_ROOT/.env"
  set +a
  export DC="${DOCKER_COMPOSE:-docker compose}"
}

inventiv_require_docker() {
  inventiv_bootstrap_docker_path || true
  if ! command -v docker >/dev/null 2>&1; then
    echo "Docker CLI not found after PATH bootstrap — this command needs Docker + Compose v2." >&2
    echo "  macOS: open Docker Desktop (whale icon), wait until it says \"running\", then retry in a new terminal:  docker version" >&2
    if [[ "$(uname -s 2>/dev/null)" == "Darwin" ]]; then
      if [[ -d "/Applications/Docker.app" ]]; then
        echo "  Docker.app is present; try adding to PATH: export PATH=\"/Applications/Docker.app/Contents/Resources/bin:\$PATH\"" >&2
        echo "  Or set INVENTIV_DOCKER_BIN=/Applications/Docker.app/Contents/Resources/bin for this repo's scripts." >&2
      fi
    fi
    echo "  Without Docker: start Postgres + \`psql\`, set POSTGRES_* + DATABASE_URL in .env (see README)." >&2
    exit 1
  fi
  if ! $DC version >/dev/null 2>&1; then
    echo "'$DC' failed. Install Docker Compose v2 or set DOCKER_COMPOSE." >&2
    exit 1
  fi
}

# True (exit 0) when docker and compose look usable — does not start containers.
inventiv_has_docker() {
  inventiv_bootstrap_docker_path || true
  command -v docker >/dev/null 2>&1 && $DC version >/dev/null 2>&1
}

inventiv_docker_up() {
  inventiv_require_docker
  inventiv_ensure_env
  cd "$INVENTIV_ROOT" || exit 1
  echo "==> Starting db + redis" >&2
  if $DC up -d --wait db redis 2>/dev/null; then
    echo "==> Postgres and Redis are healthy" >&2
    return 0
  fi
  echo "==> Compose --wait not available; starting without --wait" >&2
  $DC up -d db redis
  local i
  for i in $(seq 1 90); do
    if $DC exec -T db pg_isready -h 127.0.0.1 -p 5432 -U "${POSTGRES_USER:-inventiv_user}" -d "${POSTGRES_DB:-inventiv_agents}" >/dev/null 2>&1; then
      echo "==> Postgres is ready" >&2
      return 0
    fi
    sleep 1
  done
  echo "Postgres did not become ready." >&2
  $DC logs db --tail 80 >&2 || true
  exit 1
}

inventiv_docker_down() {
  if ! inventiv_has_docker; then
    echo "Docker/Compose not available; nothing to stop for the Compose stack." >&2
    return 0
  fi
  cd "$INVENTIV_ROOT" || exit 1
  $DC down "$@"
}

# Run apply-migrations.sh; exit 2 (schema exists) is treated as success for day-to-day workflows.
inventiv_migrate_try() {
  cd "$INVENTIV_ROOT" || exit 1
  set +e
  bash "$INVENTIV_ROOT/scripts/db/apply-migrations.sh"
  local st=$?
  set -e
  if [[ $st -eq 2 ]]; then
    echo "==> Database already initialized; skipping migrations" >&2
    return 0
  fi
  if [[ $st -ne 0 ]]; then
    return "$st"
  fi
  return 0
}
