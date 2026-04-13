#!/usr/bin/env bash
# Destroy Postgres + Redis volumes for this Compose project and bring DB back empty, then apply migrations.
# WARNING: deletes all local Docker data for inventivagents named volumes.
# Usage (repo root): ./scripts/db/reset-local-db.sh

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

DC="${DOCKER_COMPOSE:-docker compose}"

echo "==> Stopping stack and removing volumes (Postgres + Redis data will be erased)"
$DC down -v

echo "==> Starting fresh db + redis"
if $DC up -d --wait db redis 2>/dev/null; then
  :
else
  $DC up -d db redis
  for i in $(seq 1 90); do
    if $DC exec -T db pg_isready -h 127.0.0.1 -p 5432 -U "${POSTGRES_USER:-inventiv_user}" -d "${POSTGRES_DB:-inventiv_agents}" >/dev/null 2>&1; then
      break
    fi
    sleep 1
  done
fi

echo "==> Applying migrations"
exec "$ROOT/scripts/db/apply-migrations.sh"
