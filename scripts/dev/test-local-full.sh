#!/usr/bin/env bash
# Full local verification: Postgres (Docker Compose or host), migrations (best-effort),
# Rust tests, release build.
# Run from repository root: ./scripts/dev/test-local-full.sh
# Same as: ./scripts/dev/dev.sh full

set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"
# shellcheck disable=SC1091
source "$ROOT/scripts/dev/lib.sh"

inventiv_ensure_env
set -a
# shellcheck disable=SC1091
source "$ROOT/.env"
set +a

inventiv_ensure_local_database

echo "==> Applying SQL migrations (best-effort; exit 2 = schema already present)"
inventiv_migrate_try

echo "==> Running Rust unit + integration tests"
export DATABASE_URL JWT_SECRET
cargo test

echo "==> Building release binary (sanity)"
cargo build --release

echo ""
echo "==> Optional: start API in another terminal:"
echo "    ./scripts/dev/dev.sh run-rel"
echo "    curl -sS -X POST http://127.0.0.1:8080/org/register -H 'Content-Type: application/json' \\"
echo "      -d '{\"name\":\"Local Co\",\"admin_email\":\"you@local.test\",\"locale\":\"en_US\"}'"
echo ""
echo "Local full test completed successfully."
