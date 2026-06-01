# Story map viewer — how it works

The UI lives in **`../web/`**: plain **TypeScript** + **Vite** (no React) for a small, copy-paste-friendly bundle.

## Layout

1. **`src/types.ts`** — TypeScript mirrors the JSON contract (`StoryMapData`, `Card`, …).
2. **`src/render.ts`** — Builds the DOM: header, optional **persona** chips (optional **`emoji`** per persona in JSON), a **backbone row** (column titles), then for each **milestone** a full-width **swimlane**: bar (id, label, color) + a **grid** aligned on columns with cards for that milestone only. Each card carries, at a glance, a top **chip strip** (**`US.x`**, **Epic** taken from the column's `epicId`, **Milestone** taken from the lane, status), the **resumed title**, an optional `subtitle` (or a **`N technical tasks`** count derived from **`refs`** whose `label` matches `T###`), an inline **As a / I want / So that** block read from **`story.asA / iWant / soThat`** (clamped to a few lines for layout), the **personas** row (with a `Personas` label) and the **`refs`** list. The full story (DoD, AC, business rules, Spec Kit Markdown) is opened in the modal — see `src/story-modal.ts`. **`src/validate.ts`** enforces **one `userStoryId` per backlog file** (a US can be on at most one card and one milestone — split into a new `US.x` to move it).
3. **`src/main.ts`** — Imports the default backlog JSON and mounts the renderer into `#app`.
4. **`src/styles.css`** — Visual design (dark theme, horizontal milestone blocks, cards with a left accent in the lane color).
5. **`src/story-modal.ts`** — Click a card: dialog with structured **story** fields, a **Technical tasks (`T###`)** section (from **`refs`**), and **Spec Kit** tabs (Markdown loaded from `/spec-kit/specify/…`).
6. **`scripts/sync-spec-kit.mjs`** — Copies repo **`specify/`** → `web/public/spec-kit/specify/` (run with **`predev` / `prebuild`** alongside backlog sync).

## Data loading

### Default (bundled)

`main.ts` imports `inventivagents.json` from `@backlog/` so the default map is **embedded in the JS bundle** (no network round-trip). Vite resolves `@backlog` to `../backlog/` (`vite.config.ts`, `server.fs.allow`).

### Runtime: `?file=` / `?backlog=`

Open e.g. `http://127.0.0.1:5190/?file=template.min.json`. The app `fetch`es `/backlog/<filename>`.

Before **`npm run dev`** or **`npm run build`**, the lifecycle script **`predev` / `prebuild`** runs `scripts/sync-public-backlog.mjs`, which copies every `../backlog/*.json` into **`web/public/backlog/`**. Vite then serves them at `/backlog/…` (and copies them into **`dist/`** for preview or static hosting).

The query value must match `^[a-zA-Z0-9][a-zA-Z0-9_.-]*\.json$` (basename only, no `..`).

### Other options

**Change the default import** in `src/main.ts` to another `@backlog/*.json` if you always want one product map bundled without using `?file=`.

## Commands

From `tools/storymap/web/`:

```bash
npm install
npm run check:english  # heuristics: accented Latin + common French tokens in backlog / docs / src
npm run dev    # default port 5190 (see vite.config.ts)
npm run build  # runs English check in prebuild, then tsc + vite → dist/
npm run preview
```

From monorepo root (if Make targets are added):

```bash
make storymap-dev
```

## Static hosting

After `npm run build`, serve `web/dist/` with any static file server. The default view still **bundles** `inventivagents.json`. Files under **`dist/backlog/`** are present when you use **`?file=`** (thanks to `public/backlog/` sync before build). Validate JSON in the browser: failed fetch or schema-ish checks show an error panel (`renderError` in `render.ts`).

## Reuse in another repository

Copy the entire **`tools/storymap/`** directory:

- Replace **`backlog/*.json`** with your product map.
- Adjust **`web/src/main.ts`** import.
- Optionally replace **`src/styles.css`** with your brand tokens.

Keep **`docs/format.md`** and **`schema.json`** with the backlog so the contract stays documented.
