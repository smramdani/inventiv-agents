import { cardStatusLabel } from "./card-status";
import { openStoryModal } from "./story-modal";
import type { Card, Column, Milestone, Persona, StoryMapData } from "./types";

function el<K extends keyof HTMLElementTagNameMap>(
  tag: K,
  className?: string,
  text?: string,
): HTMLElementTagNameMap[K] {
  const node = document.createElement(tag);
  if (className) {
    node.className = className;
  }
  if (text !== undefined) {
    node.textContent = text;
  }
  return node;
}

function sortColumns(cols: Column[]): Column[] {
  return [...cols].sort((a, b) => a.order - b.order);
}

function sortMilestones(ms: Milestone[]): Milestone[] {
  return [...ms].sort((a, b) => a.order - b.order);
}

function sortCardsInLane(cards: Card[], columnId: string, milestoneId: string): Card[] {
  return cards
    .filter((c) => c.columnId === columnId && c.milestoneId === milestoneId)
    .sort((a, b) => a.title.localeCompare(b.title));
}

function buildPersonaMap(personas?: Persona[]): Map<string, Persona> {
  const map = new Map<string, Persona>();
  for (const p of personas ?? []) {
    map.set(p.id, p);
  }
  return map;
}

function personaTitle(p: Persona): string {
  return p.description ? `${p.label} — ${p.description}` : p.label;
}

function userStoryChipLabel(card: Card): string | null {
  if (card.userStoryId === undefined) return null;
  if (card.userStoryId === null) return "—";
  return card.userStoryId;
}

function countTechnicalTaskRefs(card: Card): number {
  return (card.refs ?? []).filter((r) => /^T\d{3}$/.test(String(r.label).trim())).length;
}

/**
 * Card subtitle below the title: either the JSON `subtitle` field (verbatim, with
 * `US.x` prepended only if missing) or, when omitted, a short technical-task count.
 * The `US.x` itself is shown as a dedicated chip in the header row, so we no longer
 * duplicate it here when a `subtitle` is provided.
 */
function formatCardSubtitle(card: Card): string | null {
  const sub = card.subtitle?.trim() ?? "";
  const n = countTechnicalTaskRefs(card);
  const taskHint = n > 0 ? `${n} technical task${n === 1 ? "" : "s"}` : "";

  if (sub.length > 0) return sub;
  return taskHint || null;
}

export function renderStoryMap(root: HTMLElement, data: StoryMapData): void {
  root.replaceChildren();

  const shell = el("div", "sm-shell");
  const header = el("header", "sm-header");
  header.appendChild(el("h1", "sm-title", data.meta.title));
  if (data.meta.description) {
    header.appendChild(el("p", "sm-desc", data.meta.description));
  }
  shell.appendChild(header);

  const personaById = buildPersonaMap(data.personas);

  if (data.personas?.length) {
    const personas = el("div", "sm-personas");
    personas.appendChild(el("span", "sm-personas-label", "Personas · "));
    for (const p of data.personas) {
      const chip = el("span", "sm-chip");
      chip.title = personaTitle(p);
      if (p.emoji) {
        const em = document.createElement("span");
        em.className = "sm-chip-emoji";
        em.textContent = p.emoji;
        em.setAttribute("aria-hidden", "true");
        chip.appendChild(em);
      }
      chip.appendChild(el("span", "sm-chip-label", p.label));
      personas.appendChild(chip);
    }
    shell.appendChild(personas);
  }

  const columns = sortColumns(data.columns);
  const milestones = sortMilestones(data.milestones);
  const colCount = columns.length;

  const scroll = el("div", "sm-board-scroll");
  const inner = el("div", "sm-board-inner");
  inner.style.setProperty("--sm-cols", String(colCount));

  const backbone = el("div", "sm-backbone-row");
  for (const col of columns) {
    const cell = el("div", "sm-backbone-cell");
    const head = el("div", "sm-backbone-head");
    head.appendChild(el("h2", "sm-backbone-title", col.label));
    if (col.epicId) {
      const epic = el("span", "sm-backbone-epic", col.epicId);
      epic.title = "Epic id — `specify/spec.md` §2";
      head.appendChild(epic);
    }
    if (col.description) {
      head.appendChild(el("p", "sm-backbone-desc", col.description));
    }
    cell.appendChild(head);
    backbone.appendChild(cell);
  }
  inner.appendChild(backbone);

  for (const ms of milestones) {
    const color = ms.color ?? "#5b8def";
    const block = el("section", "sm-milestone-block");
    block.style.setProperty("--lane-color", color);

    const bar = el("div", "sm-milestone-bar");
    const accent = el("div", "sm-milestone-bar-accent");
    bar.appendChild(accent);

    const barText = el("div", "sm-milestone-bar-text");
    const code = el("span", "sm-milestone-code", ms.id);
    code.title = "Milestone id — `specify/plan.md` §2";
    const name = el("span", "sm-milestone-name", ms.label);
    barText.appendChild(code);
    barText.appendChild(name);
    bar.appendChild(barText);
    block.appendChild(bar);

    const laneGrid = el("div", "sm-milestone-grid");
    for (const col of columns) {
      const laneCell = el("div", "sm-lane-cell");
      const stack = el("div", "sm-stack");
      for (const card of sortCardsInLane(data.cards, col.id, ms.id)) {
        stack.appendChild(renderCard(card, ms, personaById, col));
      }
      laneCell.appendChild(stack);
      laneGrid.appendChild(laneCell);
    }
    block.appendChild(laneGrid);
    inner.appendChild(block);
  }

  scroll.appendChild(inner);
  shell.appendChild(scroll);

  const foot = el("footer", "sm-foot");
  foot.textContent =
    "Story map · v2 · one card = one US (US.x · Epic · Milestone · personas · As a / I want / So that on the card; T### + Spec Kit in modal) · ?file=…";
  shell.appendChild(foot);

  root.appendChild(shell);
}

function renderCard(
  card: Card,
  laneMilestone: Milestone,
  personaById: Map<string, Persona>,
  column: Column,
): HTMLElement {
  const wrap = el("article", "sm-card sm-card--in-lane sm-card--interactive");
  wrap.style.setProperty("--milestone-color", laneMilestone.color ?? "#5b8def");
  wrap.tabIndex = 0;
  wrap.setAttribute("role", "button");
  const usForAria = userStoryChipLabel(card);
  wrap.setAttribute(
    "aria-label",
    usForAria ? `Open details: ${card.title} (${usForAria})` : `Open details: ${card.title}`,
  );
  const open = () => {
    openStoryModal({ card, columnLabel: column.label, laneMilestone, personaById });
  };
  wrap.addEventListener("click", (ev) => {
    const node = ev.target;
    const tgt = node instanceof Element ? node : (node as Node).parentElement;
    if (tgt?.closest("a")) return;
    open();
  });
  wrap.addEventListener("keydown", (ev) => {
    if (ev.key === "Enter" || ev.key === " ") {
      ev.preventDefault();
      open();
    }
  });

  const body = el("div", "sm-card-body");

  body.appendChild(renderCardChips(card, laneMilestone, column));

  body.appendChild(el("h3", "sm-card-title", card.title));

  const subLine = formatCardSubtitle(card);
  if (subLine) {
    body.appendChild(el("p", "sm-card-sub", subLine));
  }

  const storyBlock = renderCardStoryBlock(card);
  if (storyBlock) body.appendChild(storyBlock);

  if (card.personaIds?.length) {
    body.appendChild(renderCardPersonas(card, personaById));
  }

  if (card.refs?.length) {
    body.appendChild(renderCardRefs(card));
  }

  wrap.appendChild(body);
  return wrap;
}

/** Top chip strip: US.x · Epic · Milestone · Status (compact, scannable). */
function renderCardChips(card: Card, laneMilestone: Milestone, column: Column): HTMLElement {
  const row = el("div", "sm-card-chips");

  const us = userStoryChipLabel(card);
  if (us) {
    const usChip = el("span", "sm-card-chip sm-card-chip--us", us);
    usChip.title = `User story id — specify/spec.md §7 (${us})`;
    row.appendChild(usChip);
  }

  const epicId = column.epicId;
  if (epicId) {
    const epicChip = el("span", "sm-card-chip sm-card-chip--epic", epicId);
    epicChip.title = `Epic — specify/spec.md §2 (${epicId} · ${column.label})`;
    row.appendChild(epicChip);
  }

  const msChip = el("span", "sm-card-chip sm-card-chip--milestone", laneMilestone.id);
  msChip.title = `Milestone — specify/plan.md §2 (${laneMilestone.id} · ${laneMilestone.label})`;
  row.appendChild(msChip);

  const status = el("span", `sm-card-chip sm-status sm-status--${card.status}`);
  status.textContent = cardStatusLabel(card.status);
  row.appendChild(status);

  return row;
}

/** Inline "As a / I want / So that" block on the card itself (truncated for height-friendliness). */
function renderCardStoryBlock(card: Card): HTMLElement | null {
  const story = card.story;
  if (!story) return null;
  const asA = story.asA?.trim() ?? "";
  const iWant = story.iWant?.trim() ?? "";
  const soThat = story.soThat?.trim() ?? "";
  if (!asA && !iWant && !soThat) return null;

  const block = el("dl", "sm-card-story");
  appendStoryLine(block, "As a", asA);
  appendStoryLine(block, "I want", iWant);
  appendStoryLine(block, "So that", soThat);
  return block;
}

function appendStoryLine(parent: HTMLElement, label: string, value: string): void {
  if (!value) return;
  const dt = el("dt", "sm-card-story-label", label);
  const dd = el("dd", "sm-card-story-value", value);
  dd.title = value;
  parent.appendChild(dt);
  parent.appendChild(dd);
}

function renderCardPersonas(card: Card, personaById: Map<string, Persona>): HTMLElement {
  const wrap = el("div", "sm-card-personas");
  const label = el("span", "sm-card-personas-label", "Personas");
  wrap.appendChild(label);
  const row = el("div", "sm-persona-row");
  for (const pid of card.personaIds ?? []) {
    const persona = personaById.get(pid);
    const glyph = el("span", "sm-persona-glyph");
    glyph.setAttribute("role", "img");
    if (persona?.emoji) {
      glyph.textContent = persona.emoji;
      glyph.title = personaTitle(persona);
      glyph.setAttribute("aria-label", persona.label);
    } else if (persona) {
      glyph.classList.add("sm-persona-glyph--fallback");
      glyph.textContent = persona.label.charAt(0).toUpperCase();
      glyph.title = personaTitle(persona);
      glyph.setAttribute("aria-label", persona.label);
    } else {
      glyph.classList.add("sm-persona-glyph--fallback", "sm-persona-glyph--unknown");
      glyph.textContent = pid.charAt(0).toUpperCase();
      glyph.title = `Unknown persona id: ${pid}`;
      glyph.setAttribute("aria-label", pid);
    }
    row.appendChild(glyph);
  }
  wrap.appendChild(row);
  return wrap;
}

function renderCardRefs(card: Card): HTMLElement {
  const refs = el("ul", "sm-refs");
  for (const r of card.refs ?? []) {
    const li = el("li", "sm-ref");
    if (r.url) {
      const a = el("a", "sm-ref-link");
      a.href = r.url;
      a.target = "_blank";
      a.rel = "noopener noreferrer";
      a.textContent = r.label;
      li.appendChild(a);
    } else if (r.path) {
      const line = el("span", "sm-ref-line");
      line.appendChild(el("span", "sm-ref-label", `${r.label}: `));
      line.appendChild(el("code", "sm-ref-path", r.path));
      li.appendChild(line);
    } else {
      li.textContent = r.label;
    }
    refs.appendChild(li);
  }
  return refs;
}

export function renderError(root: HTMLElement, message: string): void {
  root.replaceChildren();
  const panel = el("div", "sm-error-panel");
  panel.appendChild(el("h1", "sm-error-title", "Story map"));
  panel.appendChild(el("p", "sm-error-msg", message));
  const hint = el("p", "sm-error-hint");
  hint.textContent =
    "Tip: default backlog is bundled without ?file=. With ?file=, JSON must exist under tools/storymap/backlog/ and be copied to web/public/backlog/ (npm run dev / npm run build runs sync automatically).";
  panel.appendChild(hint);
  root.appendChild(panel);
}
