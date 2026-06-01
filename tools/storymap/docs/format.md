# Story map backlog — JSON format (**v2**)

This folder describes the **contract** consumed by the viewer in `../web/`.  
Machine-readable schema: [`../backlog/schema.json`](../backlog/schema.json).  
Minimal starter file: [`../backlog/template.min.json`](../backlog/template.min.json).

**v2 breaking changes (from v1)**:

- `releases` → **`milestones`** (planning swimlanes = **`specify/plan.md` §2** milestones — not Constitution **XV** software releases).
- `cards[].releaseId` → **`milestoneId`**.
- **`cards[].userStoryId`**: **`US.1`**–**`US.15`** from **`specify/spec.md` §7**. Each **US** is bound to **exactly one milestone**, so the viewer enforces **exactly one card per `userStoryId`** (and rejects the same `userStoryId` on two milestones). Prepended to the card **subtitle** in the viewer (with optional **`subtitle`** text or **technical task count**); full **story** + **task list** in the **modal** (see `renderer.md`).
- `meta.formatVersion` must be **`2`**.

**Canonical matrix** for **US ↔ `T###` ↔ milestone**: **`specify/traceability.md`**. Edit that file first when moving a user story between milestones, then update **`specify/plan.md` §2**, task rows (**`Milestone`** column), and this JSON.

## Top-level object

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `meta` | object | yes | Project title and format version (`2`). |
| `personas` | array | no | Optional chips above the board. |
| `columns` | array | yes | Backbone steps, **left-to-right narrative**. |
| `milestones` | array | yes | Swimlanes / **M5a**, **M5b**, … |
| `cards` | array | yes | Work items placed under a column + milestone. |

### `meta`

- `title` (string): shown in the viewer header.
- `formatVersion` (integer): must be **`2`** for this spec revision.
- `description` (string, optional): subtitle text.

### `personas[]`

- `id` (string): stable id, referenced by cards.
- `label` (string): short display name.
- `emoji` (string, optional): pictogram shown on the persona chips and on each card’s persona row (typically one emoji; multi-codepoint sequences are allowed).
- `description` (string, optional): tooltip on chips and persona glyphs.

### `columns[]`

- `id` (string): unique key; **`cards[].columnId` must match**.
- `label` (string): column header.
- `order` (integer): horizontal sort (ascending).
- `description` (string, optional): helper text under the title.
- `epicId` (string, optional): id from **`specify/spec.md` §2** (e.g. **E-ONB**). Shown under the column title in the viewer; backbone should follow **`spec.md` §2.1** order left → right.

### `milestones[]`

- `id` (string): unique key; **`cards[].milestoneId` must match** (e.g. **M5a**).
- `label` (string): lane title in the viewer.
- `order` (integer): vertical sort of swimlanes (lower first).
- `color` (string, optional): CSS color for the card ribbon (e.g. `#5b8def`).

### `cards[]`

- `id` (string): unique card id.
- `columnId` (string): **Epic** column for this **user story** — must match the **owning Epic** for that **`US.x`** in **`specify/spec.md` §7.0** (e.g. **US.5** → **E-REG** → `registry`).
- `title` (string): **User story** title (product outcome), not a raw **`T###`** title.
- `subtitle` (string, optional): extra line on the tile; if omitted, the viewer may show **`US.x · N technical tasks`** from **`refs`**.
- `milestoneId` (string): which **milestone** swimlane the card belongs to.
- `userStoryId` (string, required for shipped/planned cards): **`US.1`** … **`US.15`** from **`specify/spec.md` §7** only. Each **US** belongs to **exactly one milestone**, so each **`userStoryId`** appears on **exactly one card** per backlog file. Each **`T###`** in **`refs`** maps to **exactly one** **`US.x`** in task files (**Constitution XII**). **`refs`** lists **all** **`T###`** that implement this **US**. **Reprioritisation**: move the card to the new milestone (and update task **Milestone** rows + **`traceability.md`**); **never** duplicate the **`US.x`** across milestones — split into a new `US.x` instead.
- `status` (string): one of `planned` | `in_progress` | `shipped` | `deferred`.
- `personaIds` (string[], optional): subset of persona ids.
- `refs` (array, optional): traceability to Spec Kit or code — include **`T###`** task rows (`label` + `path` to `specify/tasks/*.md` where helpful).
  - `label` (string): human-readable ref name.
  - `path` (string, optional): repo-relative path (display only in browser).
  - `url` (string, optional): external link (opens in new tab).
- `story` (object, optional): **User story** + acceptance fields shown in the card modal; **Spec Kit sync** below.
  - `asA`, `iWant`, `soThat` (strings, optional): classic user story lines.
  - `explanationMarkdown` (string, optional): extra context (Markdown).
  - `businessRules`, `definitionOfDone`, `acceptanceCriteria` (string arrays, optional): bullet lists in the modal.
  - `specPaths` (string[], optional): extra repo-relative paths to **`specify/**/*.md`** only. At **open card**, the viewer **fetches** and renders Markdown for:
    - every path in `specPaths`, **plus**
    - every `refs[].path` that points to `specify/**/*.md`  
    (deduped). Requires `npm run dev` / `npm run build` so `specify/` is copied to `web/public/spec-kit/` (see `renderer.md`).

## One card per user story (required)

**One card = one `US.x`**. Each **`US.x`** is bound to **one milestone** (one Epic + one milestone), so the card naturally lives on a single swimlane. Decompose delivery with **`T###`** rows in **`specify/tasks/*.md`** and list them all on that card’s **`refs`**; the **modal** shows the **Technical tasks** list. Do **not** add a second card for the same `userStoryId`, and do **not** reuse the same `userStoryId` on a different milestone — **split** the theme into a new `US.x` (e.g. `US.7` → `US.11` → `US.12`) instead. **Governance** when scope shifts: **`specify/spec.md` §2 / §7 / §10**, **`specify/plan.md` §2 / §5**, **`specify/traceability.md`**, task **Milestone** / **Epic** columns, and this JSON in the **same** PR (**Constitution XVI**).

## Validation rules

1. Every `cards[].columnId` exists in `columns[].id`.
2. Every `cards[].milestoneId` exists in `milestones[].id`.
3. `columns` and `milestones` should each contain at least one item.
4. **Each `userStoryId` appears on at most one card** (and therefore at most one milestone). Reusing a `userStoryId` on a second card — same or different milestone — is rejected with an error pointing at this rule.

## Adding a second backlog file

1. Copy `backlog/inventivagents.json` to e.g. `backlog/other-product.json`.
2. In another project, copy the whole `tools/storymap` tree and replace the JSON.
3. Either change the default import in `../web/src/main.ts`, or keep the default and open the viewer with **`?file=your-map.json`** after placing the file under `backlog/` (see `renderer.md` for sync to `public/backlog/`).

## Versioning

**v1** (`releases` / `releaseId`) is **retired** in this repo — use **v2** only. Bump `meta.formatVersion` and extend `schema.json` when making future breaking changes; keep this document in sync (**Constitution XII-style** traceability for the tool itself).
