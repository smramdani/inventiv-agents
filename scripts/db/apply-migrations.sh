#!/usr/bin/env bash
# Apply SQL migrations in lexical order against the Postgres instance from docker-compose.
# Usage (from repo root):
#   ./scripts/db/apply-migrations.sh
# Requires: docker compose, running "db" service, POSTGRES_* in .env (see .env.example).

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

if [[ -f .env ]]; then
  set -a
  # shellcheck disable=SC1091
  source .env
  set +a
fi

if ! docker compose exec -T db pg_isready -U "${POSTGRES_USER:-inventiv_user}" -d "${POSTGRES_DB:-inventiv_agents}" >/dev/null 2>&1; then
  echo "Postgres is not reachable. Start the stack with: docker compose up -d db" >&2
  exit 1
fi

shopt -s nullglob
files=(migrations/*.sql)
if [[ ${#files[@]} -eq 0 ]]; then
  echo "No migrations/*.sql files found." >&2
  exit 1
fi

for f in "${files[@]}"; do
  echo "==> Applying $(basename "$f")"
  docker compose exec -T db psql \
    -v ON_ERROR_STOP=1 \
    -U "${POSTGRES_USER:-inventiv_user}" \
    -d "${POSTGRES_DB:-inventiv_agents}" \
    <"$f"
done

echo "==> Migrations finished."
