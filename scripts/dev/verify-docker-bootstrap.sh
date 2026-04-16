#!/usr/bin/env bash
# Self-test: with a stripped PATH, INVENTIV_DOCKER_BIN must make inventiv_has_docker succeed
# using the same lib.sh bootstrap as make/dev.sh.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

mkdir -p "$TMP"
cat >"$TMP/docker" <<'STUB'
#!/usr/bin/env bash
# Minimal stub so `docker version` and `docker compose version` succeed.
if [[ "${1:-}" == "compose" ]]; then
  shift
  case "${1:-}" in
    version) echo "Docker Compose version test-stub" ;;
    *) exit 0 ;;
  esac
  exit 0
fi
if [[ "${1:-}" == "version" ]]; then
  echo "Docker version test-stub"
  exit 0
fi
exit 0
STUB
chmod +x "$TMP/docker"

export INVENTIV_DOCKER_BIN="$TMP"
export PATH="/usr/bin:/bin"
unset DOCKER_COMPOSE

# shellcheck disable=SC1091
source "$ROOT/scripts/dev/lib.sh"

if ! command -v docker >/dev/null 2>&1; then
  echo "FAIL: docker not on PATH after bootstrap (INVENTIV_DOCKER_BIN=$INVENTIV_DOCKER_BIN)" >&2
  exit 1
fi

if ! inventiv_has_docker; then
  echo "FAIL: inventiv_has_docker should succeed with stub docker + compose" >&2
  exit 1
fi

echo "OK: Docker PATH bootstrap + inventiv_has_docker (stubbed CLI)"
