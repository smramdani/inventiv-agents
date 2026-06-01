# Story map (local mini-tool)

Modular, **copy-paste reusable** story map: **JSON backlog** + **TypeScript/HTML viewer** + **documentation**.

```
tools/storymap/
  backlog/           # JSON data (+ JSON Schema)
  docs/              # Format + renderer documentation
  web/               # Vite + TS viewer (no framework)
  README.md          # This file
```

## Quick start

```bash
cd tools/storymap/web
npm install
npm run check:english   # optional: verify backlog + docs + viewer sources are English-only
npm run dev
```

`npm run build` runs the same English check automatically (after syncing backlog + `specify/`).

Open the printed URL (default **http://127.0.0.1:5190**).

`npm run dev` / `npm run build` run two syncs: **`backlog/*.json`** → `web/public/backlog/`, and repo **`specify/`** → `web/public/spec-kit/specify/` (for the **card modal**: live Markdown from Spec Kit).

Open **`?file=template.min.json`** (or any backlog filename) without changing code.

**User stories**: each tile is **one `US.x`** (and a US belongs to a single milestone); click to read `story.*`, the **Technical tasks (`T###`)** list from `refs`, and fetched **`specify/**/*.md`** (merged with `refs[].path` when those paths are Spec Kit markdown).

## Edit the map

- Data: **`backlog/inventivagents.json`**
- Contract: **`backlog/schema.json`** + **`docs/format.md`**
- UI behaviour / build: **`docs/renderer.md`**

## Spec Kit

This tool **visualizes** backlog data; it does not replace **`specify/spec.md`** (**§2 Epics**, **§7 User Stories** — **`US.1`–`US.15`**, **§7.0** epic map + **§7.1–7.15** narratives), **`specify/traceability.md`** (**US ↔ `T###` ↔ milestone** + epic→US table), **`plan.md`** (**§2** journey-aligned **milestone** backlog; **§2.0.2** epic↔US), or **`specify/tasks/*.md`**. Backbone **`columns[].epicId`** should match **`spec.md` §2** (viewer shows the id under each column title). **Each `US.x` belongs to exactly one milestone**, so the viewer shows **exactly one card per `userStoryId`** with **`refs`** listing every **`T###`** for that **US** (**Constitution XII** / **XVI**). JSON format **v2**: `tools/storymap/docs/format.md`.
