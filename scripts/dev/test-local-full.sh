#!/usr/bin/env bash
# Full local verification: Docker Postgres + Redis, migrations, Rust tests, release build.
# Run from repository root: ./scripts/dev/test-local-full.sh
# Prerequisites: Docker Desktop / Engine + Compose v2 (supports `compose up --wait`).

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

DC="${DOCKER_COMPOSE:-docker compose}"

if ! command -v docker >/dev/null 2>&1; then
  echo "Docker is not installed or not on PATH. Install Docker Desktop and retry." >&2
  exit 1
fi

if ! $DC version >/dev/null 2>&1; then
  echo "'$DC' failed. Install Docker Compose v2 (plugin) or set DOCKER_COMPOSE to your wrapper." >&2
  exit 1
fi

if [[ ! -f .env ]]; then
  echo "No .env file; copying .env.example to .env (edit secrets before shared machines)."
  cp .env.example .env
fi

set -a
# shellcheck disable=SC1091
source .env
set +a

echo "==> Starting db + redis (wait until healthy)"
if $DC up -d --wait db redis 2>/dev/null; then
  echo "Services are up and passed healthchecks."
else
  echo "Note: 'docker compose up --wait' not supported by this Compose version; starting without --wait."
  $DC up -d db redis
  echo "==> Waiting for Postgres (pg_isready)"
  for i in $(seq 1 90); do
    if $DC exec -T db pg_isready -h 127.0.0.1 -p 5432 -U "${POSTGRES_USER:-inventiv_user}" -d "${POSTGRES_DB:-inventiv_agents}" >/dev/null 2>&1; then
      echo "Postgres is ready."
      break
    fi
    if [[ "$i" -eq 90 ]]; then
      echo "Postgres did not become ready in time." >&2
      $DC logs db --tail 80 >&2 || true
      exit 1
    fi
    sleep 1
  done
fi

echo "==> Applying SQL migrations"
"$ROOT/scripts/db/apply-migrations.sh"

echo "==> Running Rust unit + integration tests"
export DATABASE_URL JWT_SECRET
cargo test

echo "==> Building release binary (sanity)"
cargo build --release

echo ""
echo "==> Optional: start API in another terminal and smoke-test:"
echo "    set -a && source .env && set +a && cargo run --release"
echo "    curl -sS -X POST http://127.0.0.1:8080/org/register -H 'Content-Type: application/json' \\"
echo "      -d '{\"name\":\"Local Co\",\"admin_email\":\"you@local.test\",\"locale\":\"en_US\"}'"
echo ""
echo "Local full test completed successfully."
