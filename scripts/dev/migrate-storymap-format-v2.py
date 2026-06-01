#!/usr/bin/env python3
"""Migrate tools/storymap/backlog/*.json from format v1 to v2 (milestones, milestoneId, userStoryId)."""
from __future__ import annotations

import json
import pathlib
import sys

REPO = pathlib.Path(__file__).resolve().parents[2]
BACKLOG = REPO / "tools" / "storymap" / "backlog"

# Card id -> spec userStoryId (null = platform / no numbered US)
USER_STORY_BY_CARD: dict[str, str | None] = {
    "m5a-register-org": "US.4",
    "m5a-cockpit-repo-layout": None,
    "m5a-login-jwt-session": "US.4",
    "m5a-whoami-role": "US.4",
    "m5a-list-llm-providers": "US.1",
    "m5a-create-llm-provider": "US.1",
    "m5a-list-skills": "US.2",
    "m5a-create-skill": "US.2",
    "m5a-list-agents": "US.3",
    "m5a-create-agent": "US.3",
    "m5a-ephemeral-sse-completion": "US.4",
    "m5a-capture-usage-from-sse": "US.5",
    "m5a-owner-usage-panel": "US.5",
    "m5a-fe-tooling-docs": None,
    "m5b-sessions-schema-rls": "US.4",
    "m5b-session-api-crud": "US.4",
    "m5b-session-fe-resume-sse": "US.4",
    "m5b-session-group-share": "US.4",
    "m5b-metrics-client-contract": "US.5",
    "m5b-owner-dashboard-ui": "US.5",
    "m5b-invite-users-ui": "US.4",
    "m5b-manage-groups-ui": "US.4",
    "m4b-execution-metrics-migration": "US.5",
    "m4b-execution-metrics-repository": "US.5",
    "m4b-link-stream-to-persisted-run": "US.5",
    "m4b-mcp-tool-in-product-loop": "US.2",
    "m4b-reasoning-orchestrator-service": "US.3",
    "m4b-orchestrated-stream-api": "US.4",
    "m4b-release-engineering-docs": None,
}


def migrate(path: pathlib.Path) -> bool:
    raw = path.read_text(encoding="utf-8")
    d = json.loads(raw)
    fv = d.get("meta", {}).get("formatVersion")
    if fv == 2:
        return False
    if fv != 1:
        raise SystemExit(f"{path}: expected meta.formatVersion 1, got {fv!r}")

    d["meta"]["formatVersion"] = 2
    if "releases" in d:
        d["milestones"] = d.pop("releases")
    for c in d.get("cards", []):
        if "releaseId" in c:
            c["milestoneId"] = c.pop("releaseId")
        cid = c.get("id", "")
        uid = USER_STORY_BY_CARD.get(cid)
        if uid is not None:
            c["userStoryId"] = uid
        elif cid in USER_STORY_BY_CARD:
            c["userStoryId"] = None
        # inventivagents: all ids mapped; template: skip if unknown
    path.write_text(json.dumps(d, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    return True


def main() -> int:
    changed = 0
    for p in sorted(BACKLOG.glob("*.json")):
        if p.name == "schema.json":
            continue
        if migrate(p):
            print("migrated", p.relative_to(REPO))
            changed += 1
    print("files migrated:", changed)
    return 0


if __name__ == "__main__":
    sys.exit(main())
