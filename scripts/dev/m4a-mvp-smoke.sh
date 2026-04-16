#!/usr/bin/env bash
# End-to-end M4a MVP smoke: register → login → provider (with key) → agent → SSE completion.
# Spec Kit: specify/mvp-engine-validation.md (manual section).
#
# Prerequisites: API running (e.g. make run), DB migrated, JWT_SECRET set for the API process.
#
# Usage:
#   export M4A_LLM_API_KEY="sk-..."   # test key; never commit
#   ./scripts/dev/m4a-mvp-smoke.sh
#
# Optional env:
#   M4A_API_BASE        default http://127.0.0.1:8080
#   M4A_LLM_BASE_URL    default https://api.openai.com
#   M4A_LLM_MODEL       default gpt-4o-mini
#   M4A_ADMIN_EMAIL     default unique m4a_<epoch>@example.com
#   M4A_ORG_NAME        default "M4a MVP Smoke"

set -euo pipefail

command -v curl >/dev/null 2>&1 || {
  echo "error: curl is required" >&2
  exit 1
}
command -v python3 >/dev/null 2>&1 || {
  echo "error: python3 is required (JSON parsing)" >&2
  exit 1
}

BASE="${M4A_API_BASE:-http://127.0.0.1:8080}"
BASE="${BASE%/}"
EMAIL="${M4A_ADMIN_EMAIL:-m4a_$(date +%s)_smoke@example.com}"
ORG_NAME="${M4A_ORG_NAME:-M4a MVP Smoke}"
LLM_BASE="${M4A_LLM_BASE_URL:-https://api.openai.com}"
LLM_BASE="${LLM_BASE%/}"
MODEL="${M4A_LLM_MODEL:-gpt-4o-mini}"

if [[ -z "${M4A_LLM_API_KEY:-}" ]]; then
  echo "error: export M4A_LLM_API_KEY with a test provider secret (never commit keys)." >&2
  echo "  optional: M4A_LLM_BASE_URL (default ${LLM_BASE}), M4A_LLM_MODEL (default ${MODEL})" >&2
  exit 2
fi

HDR=$(mktemp)
BODY=$(mktemp)
trap 'rm -f "$HDR" "$BODY"' EXIT

json_register() {
  python3 -c 'import json,sys; print(json.dumps({"name":sys.argv[1],"admin_email":sys.argv[2],"locale":"en_US"}))' "$ORG_NAME" "$EMAIL"
}

json_login() {
  python3 -c 'import json,sys; print(json.dumps({"email":sys.argv[1]}))' "$EMAIL"
}

json_provider() {
  python3 -c 'import json,sys; print(json.dumps({"name":"M4a Smoke Provider","base_url":sys.argv[1],"api_key":sys.argv[2]}))' "$LLM_BASE" "$M4A_LLM_API_KEY"
}

json_agent() {
  python3 -c 'import json,sys; print(json.dumps({"name":"M4a Smoke Agent","mission":"One-shot MVP validation","llm_provider_id":sys.argv[1]}))' "$1"
}

json_stream() {
  python3 -c 'import json,sys; print(json.dumps({"message":"Reply with exactly one English word: OK.","model":sys.argv[1],"max_tokens":64}))' "$MODEL"
}

echo "==> M4a smoke against ${BASE} (admin ${EMAIL})"

echo "==> POST /org/register"
curl -sfS -X POST "${BASE}/org/register" \
  -H 'Content-Type: application/json' \
  -d "$(json_register)" >/dev/null

echo "==> POST /auth/login"
LOGIN_JSON=$(curl -sfS -X POST "${BASE}/auth/login" \
  -H 'Content-Type: application/json' \
  -d "$(json_login)")
TOKEN=$(echo "$LOGIN_JSON" | python3 -c "import json,sys; print(json.load(sys.stdin)['token'])")

echo "==> POST /org/providers"
PROV_JSON=$(curl -sfS -X POST "${BASE}/org/providers" \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer ${TOKEN}" \
  -d "$(json_provider)")
PROVIDER_ID=$(echo "$PROV_JSON" | python3 -c "import json,sys; print(json.load(sys.stdin)['id'])")

echo "==> POST /org/agents"
AGENT_JSON=$(curl -sfS -X POST "${BASE}/org/agents" \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer ${TOKEN}" \
  -d "$(json_agent "$PROVIDER_ID")")
AGENT_ID=$(echo "$AGENT_JSON" | python3 -c "import json,sys; print(json.load(sys.stdin)['id'])")

TRACE=$(python3 -c "import uuid; print(uuid.uuid4())")
echo "==> POST /org/agents/${AGENT_ID}/complete/stream (SSE, X-Trace-ID: ${TRACE})"

curl -sfS -N -D "$HDR" -o "$BODY" \
  -X POST "${BASE}/org/agents/${AGENT_ID}/complete/stream" \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer ${TOKEN}" \
  -H "X-Trace-ID: ${TRACE}" \
  -d "$(json_stream)"

if ! grep -qi 'content-type: text/event-stream' "$HDR"; then
  echo "error: expected Content-Type text/event-stream in response headers" >&2
  grep -i content-type "$HDR" >&2 || true
  exit 1
fi

if ! grep -qi '^x-trace-id:' "$HDR"; then
  echo "error: expected X-Trace-ID response header" >&2
  exit 1
fi

if ! grep -q '^event: meta' "$BODY"; then
  echo "error: SSE body missing event: meta" >&2
  exit 1
fi
if ! grep -q '^event: delta' "$BODY"; then
  echo "error: SSE body missing event: delta (LLM or auth failure?)" >&2
  grep '^event:' "$BODY" >&2 || true
  exit 1
fi
if ! grep -q '^event: usage' "$BODY"; then
  echo "error: SSE body missing event: usage" >&2
  exit 1
fi
if ! grep -q '^event: done' "$BODY"; then
  echo "error: SSE body missing terminal event: done" >&2
  exit 1
fi

echo "==> M4a smoke OK (SSE meta → delta → usage → done, trace + stream headers validated)"
