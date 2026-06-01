#!/usr/bin/env python3
"""
One-off / maintenance: replace legacy task ids (T1.1, T4.3, T5.3a, …) with global T### ids.
Run from repo root: python3 scripts/dev/renumber-spec-tasks-once.py

**Do not re-run** for routine edits — once legacy strings are gone, the script is a no-op; keep it only as a record of the mapping.

Canonical allocation (InventivAgents):
  T001–T005   → `specify/tasks/001_milestone_1.md` (historical scaffold)
  T006–T014   → `specify/tasks/003_milestone_3.md` (registry)
  T015–T032   → `specify/tasks/004_milestone_4.md` (engine / M4a–M4b)
  T033–T054   → `specify/tasks/005_milestone_5.md` (cockpit M5a–M5b)

New work: assign **T055** onward; keep **Release** / plan / story map in sync (`specify/plan.md` §2.0).
"""
from __future__ import annotations

import pathlib
import sys

REPO = pathlib.Path(__file__).resolve().parents[2]
# Typographic en dash (U+2013) is used in Spec Kit markdown for ranges.
ND = "\u2013"

# Longest-first within atoms; compounds run before atoms.
COMPOUNDS: list[tuple[str, str]] = [
    ("T5.2a" + ND + "T5.2c", "T034" + ND + "T036"),
    ("T5.3a" + ND + "T5.3f", "T037" + ND + "T042"),
    ("T5.3a" + ND + "f", "T037" + ND + "T042"),
    ("T5.2a" + ND + "c", "T034" + ND + "T036"),
    ("T5.5a" + ND + "T5.5b", "T044" + ND + "T045"),
    ("T5.5a" + ND + "b", "T044" + ND + "T045"),
    ("T5.7" + ND + "T5.9", "T047" + ND + "T049"),
    ("T5.7" + ND + "T5.10", "T047" + ND + "T052"),
    ("T5.12" + ND + "T5.13", "T050" + ND + "T051"),
    ("T5.11a" + ND + "T5.11b", "T053" + ND + "T054"),
    ("T5.11a" + ND + "b", "T053" + ND + "T054"),
    ("`T4.10`" + ND + "`T4.12`", "`T024`" + ND + "`T026`"),
    ("`T4.13`" + ND + "`T4.15`", "`T027`" + ND + "`T029`"),
]

ATOMS: list[tuple[str, str]] = [
    ("T5.101", "T033"),
    ("T5.11b", "T054"),
    ("T5.11a", "T053"),
    ("T5.12", "T050"),
    ("T5.13", "T051"),
    ("T5.10", "T052"),
    ("T5.2c", "T036"),
    ("T5.2b", "T035"),
    ("T5.2a", "T034"),
    ("T5.3a", "T037"),
    ("T5.3b", "T038"),
    ("T5.3c", "T039"),
    ("T5.3d", "T040"),
    ("T5.3e", "T041"),
    ("T5.3f", "T042"),
    ("T5.5a", "T044"),
    ("T5.5b", "T045"),
    ("T5.6", "T046"),
    ("T5.7", "T047"),
    ("T5.8", "T048"),
    ("T5.9", "T049"),
    ("T5.4", "T043"),
    ("T4.18", "T032"),
    ("T4.17", "T031"),
    ("T4.16", "T030"),
    ("T4.15", "T029"),
    ("T4.14", "T028"),
    ("T4.13", "T027"),
    ("T4.12", "T026"),
    ("T4.11", "T025"),
    ("T4.10", "T024"),
    ("T4.9", "T023"),
    ("T4.8", "T022"),
    ("T4.7", "T021"),
    ("T4.6", "T020"),
    ("T4.5", "T019"),
    ("T4.4", "T018"),
    ("T4.3", "T017"),
    ("T4.2", "T016"),
    ("T4.1", "T015"),
    ("T3.1a", "T006"),
    ("T3.1b", "T007"),
    ("T3.1c", "T008"),
    ("T3.1d", "T009"),
    ("T3.2a", "T010"),
    ("T3.2b", "T011"),
    ("T3.2c", "T012"),
    ("T3.3", "T013"),
    ("T3.4", "T014"),
    ("T1.5", "T005"),
    ("T1.4", "T004"),
    ("T1.3", "T003"),
    ("T1.2", "T002"),
    ("T1.1", "T001"),
]

TEXT_EXTENSIONS = {".md", ".json"}


def subst(text: str) -> str:
    for old, new in COMPOUNDS:
        text = text.replace(old, new)
    for old, new in ATOMS:
        text = text.replace(old, new)
    return text


def main() -> int:
    targets: list[pathlib.Path] = []
    specify = REPO / "specify"
    targets.extend(specify.rglob("*.md"))
    targets.append(REPO / "tools" / "storymap" / "backlog" / "inventivagents.json")
    targets.append(REPO / "tools" / "storymap" / "docs" / "format.md")
    targets.append(REPO / "CHANGELOG.md")
    mem = REPO / ".specify" / "memory" / "constitution.md"
    if mem.is_file():
        targets.append(mem)

    changed = 0
    for path in targets:
        if not path.is_file():
            continue
        raw = path.read_text(encoding="utf-8")
        new = subst(raw)
        if new != raw:
            path.write_text(new, encoding="utf-8", newline="\n")
            changed += 1
            print("updated", path.relative_to(REPO))
    print("files changed:", changed)
    return 0


if __name__ == "__main__":
    sys.exit(main())
