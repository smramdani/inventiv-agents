#!/usr/bin/env bash
# Apply SQL migrations in lexical order against the Postgres instance from docker-compose.
# Usage (from repo root):
#   ./scripts/db/apply-migrations.sh
# Requires: docker compose, running "db" service, POSTGRES_* in .env (see .env.example).

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

DC="${DOCKER_COMPOSE:-docker compose}"

if [[ -f .env ]]; then
  set -a
  # shellcheck disable=SC1091
  source .env
  set +a
fi

if ! $DC exec -T db pg_isready -h 127.0.0.1 -p 5432 -U "${POSTGRES_USER:-inventiv_user}" -d "${POSTGRES_DB:-inventiv_agents}" >/dev/null 2>&1; then
  echo "Postgres is not reachable. Start the stack with: docker compose up -d db" >&2
  exit 1
fi

if $DC exec -T db psql -U "${POSTGRES_USER:-inventiv_user}" -d "${POSTGRES_DB:-inventiv_agents}" -tAc "SELECT 1 FROM information_schema.tables WHERE table_schema='public' AND table_name='organizations'" 2>/dev/null | grep -qx 1; then
  echo "Database already has schema (e.g. organizations table). Migrations are not idempotent on re-run." >&2
  echo "For a clean local DB: ./scripts/db/reset-local-db.sh" >&2
  exit 2
fi

shopt -s nullglob
files=(migrations/*.sql)
if [[ ${#files[@]} -eq 0 ]]; then
  echo "No migrations/*.sql files found." >&2
  exit 1
fi

for f in "${files[@]}"; do
  echo "==> Applying $(basename "$f")"
  $DC exec -T db psql \
    -v ON_ERROR_STOP=1 \
    -U "${POSTGRES_USER:-inventiv_user}" \
    -d "${POSTGRES_DB:-inventiv_agents}" \
    <"$f"
done

echo "==> Migrations finished."
