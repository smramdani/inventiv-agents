#!/usr/bin/env bash
# Apply SQL migrations in lexical order.
#
# Connection (first that works):
#   1) Docker Compose service "db" when the container is up and healthy.
#   2) Host "psql" using MIGRATE_DATABASE_URL or POSTGRES_* (superuser; required for extensions / roles).
#
# Usage (from repo root): ./scripts/db/apply-migrations.sh
# Exit 2: schema already present (treated as success by migrate_try in dev scripts).

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

DC="${DOCKER_COMPOSE:-docker compose}"

if [[ -f .env ]]; then
  set -a
  # shellcheck disable=SC1091
  source "$ROOT/.env"
  set +a
fi

docker_db_ready() {
  command -v docker >/dev/null 2>&1 \
    && $DC version >/dev/null 2>&1 \
    && $DC exec -T db pg_isready -h 127.0.0.1 -p 5432 -U "${POSTGRES_USER:-inventiv_user}" -d "${POSTGRES_DB:-inventiv_agents}" >/dev/null 2>&1
}

schema_exists_docker() {
  $DC exec -T db psql -U "${POSTGRES_USER:-inventiv_user}" -d "${POSTGRES_DB:-inventiv_agents}" -tAc "SELECT 1 FROM information_schema.tables WHERE table_schema='public' AND table_name='organizations'" 2>/dev/null | grep -qx 1
}

schema_exists_psql() {
  local out
  if [[ -n "${MIGRATE_DATABASE_URL:-}" ]]; then
    out=$(psql "$MIGRATE_DATABASE_URL" -tAc "SELECT 1 FROM information_schema.tables WHERE table_schema='public' AND table_name='organizations'" 2>/dev/null || true)
  else
    out=$(
      PGPASSWORD="${POSTGRES_PASSWORD:-inventiv_password}" psql \
        -h "${POSTGRES_HOST:-127.0.0.1}" -p "${POSTGRES_PORT:-5432}" \
        -U "${POSTGRES_USER:-inventiv_user}" -d "${POSTGRES_DB:-inventiv_agents}" \
        -tAc "SELECT 1 FROM information_schema.tables WHERE table_schema='public' AND table_name='organizations'" 2>/dev/null || true
    )
  fi
  [[ "$out" == "1" ]]
}

psql_ping() {
  if [[ -n "${MIGRATE_DATABASE_URL:-}" ]]; then
    psql "$MIGRATE_DATABASE_URL" -tAc "SELECT 1" >/dev/null 2>&1
  else
    PGPASSWORD="${POSTGRES_PASSWORD:-inventiv_password}" psql \
      -h "${POSTGRES_HOST:-127.0.0.1}" -p "${POSTGRES_PORT:-5432}" \
      -U "${POSTGRES_USER:-inventiv_user}" -d "${POSTGRES_DB:-inventiv_agents}" \
      -tAc "SELECT 1" >/dev/null 2>&1
  fi
}

apply_one_docker() {
  local f="$1"
  echo "==> Applying $(basename "$f")"
  $DC exec -T db psql \
    -v ON_ERROR_STOP=1 \
    -U "${POSTGRES_USER:-inventiv_user}" \
    -d "${POSTGRES_DB:-inventiv_agents}" \
    <"$f"
}

apply_one_psql() {
  local f="$1"
  echo "==> Applying $(basename "$f") (host psql)"
  if [[ -n "${MIGRATE_DATABASE_URL:-}" ]]; then
    psql "$MIGRATE_DATABASE_URL" -v ON_ERROR_STOP=1 -f "$f"
  else
    PGPASSWORD="${POSTGRES_PASSWORD:-inventiv_password}" psql \
      -h "${POSTGRES_HOST:-127.0.0.1}" -p "${POSTGRES_PORT:-5432}" \
      -U "${POSTGRES_USER:-inventiv_user}" -d "${POSTGRES_DB:-inventiv_agents}" \
      -v ON_ERROR_STOP=1 -f "$f"
  fi
}

shopt -s nullglob
files=(migrations/*.sql)
if [[ ${#files[@]} -eq 0 ]]; then
  echo "No migrations/*.sql files found." >&2
  exit 1
fi

if docker_db_ready; then
  echo "==> Applying migrations via Docker Compose (db service)" >&2
  if schema_exists_docker; then
    echo "Database already has schema (e.g. organizations table). Migrations are not idempotent on re-run." >&2
    echo "For a clean local DB: ./scripts/db/reset-local-db.sh" >&2
    exit 2
  fi
  for f in "${files[@]}"; do
    apply_one_docker "$f"
  done
  echo "==> Migrations finished."
  exit 0
fi

echo "==> Compose db not reachable; trying host psql (MIGRATE_DATABASE_URL or POSTGRES_*)..." >&2
if ! command -v psql >/dev/null 2>&1; then
  echo "Postgres is not reachable via Docker, and \`psql\` is not on PATH." >&2
  echo "  Install Docker Desktop, or install PostgreSQL client tools (e.g. macOS: brew install libpq && brew link --force libpq)." >&2
  exit 1
fi

if ! psql_ping; then
  echo "Cannot connect with host psql. Check POSTGRES_HOST/POSTGRES_PORT/POSTGRES_USER/POSTGRES_PASSWORD/POSTGRES_DB or set MIGRATE_DATABASE_URL." >&2
  exit 1
fi

if schema_exists_psql; then
  echo "Database already has schema (e.g. organizations table). Migrations are not idempotent on re-run." >&2
  echo "For a clean local DB: wipe the database or use Docker reset: ./scripts/db/reset-local-db.sh" >&2
  exit 2
fi

for f in "${files[@]}"; do
  apply_one_psql "$f"
done

echo "==> Migrations finished."
