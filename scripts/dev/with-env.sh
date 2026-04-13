#!/usr/bin/env bash
# Run a command with repository .env loaded (same as local `cargo run` / `cargo test`).
# Usage (from repo root): ./scripts/dev/with-env.sh cargo test
# Usage: ./scripts/dev/with-env.sh cargo clippy --all-targets

set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"
# shellcheck disable=SC1091
source "$ROOT/scripts/dev/lib.sh"
inventiv_load_env
exec "$@"
