import { cardStatusLabel } from "./card-status";
import { renderMarkdown } from "./markdown-render";
import { isSpecKitMarkdownRepoPath, specKitFetchUrl, specMarkdownPathsForCard } from "./spec-paths";
import type { Card, Milestone, Persona } from "./types";

let overlayEl: HTMLDivElement | null = null;
let loadAbort: AbortController | null = null;
let keyHandler: ((ev: KeyboardEvent) => void) | null = null;

function el<K extends keyof HTMLElementTagNameMap>(
  tag: K,
  className?: string,
  text?: string,
): HTMLElementTagNameMap[K] {
  const node = document.createElement(tag);
  if (className) node.className = className;
  if (text !== undefined) node.textContent = text;
  return node;
}

function personaGlyph(persona: Persona | undefined, pid: string): HTMLElement {
  const glyph = el("span", "sm-persona-glyph sm-modal-persona-glyph");
  glyph.setAttribute("role", "img");
  if (persona?.emoji) {
    glyph.textContent = persona.emoji;
    glyph.title = persona.description ? `${persona.label} — ${persona.description}` : persona.label;
    glyph.setAttribute("aria-label", persona.label);
  } else if (persona) {
    glyph.classList.add("sm-persona-glyph--fallback");
    glyph.textContent = persona.label.charAt(0).toUpperCase();
    glyph.title = persona.description ? `${persona.label} — ${persona.description}` : persona.label;
    glyph.setAttribute("aria-label", persona.label);
  } else {
    glyph.classList.add("sm-persona-glyph--fallback", "sm-persona-glyph--unknown");
    glyph.textContent = pid.charAt(0).toUpperCase();
    glyph.title = `Unknown persona: ${pid}`;
    glyph.setAttribute("aria-label", pid);
  }
  return glyph;
}

function section(title: string, body: HTMLElement): HTMLElement {
  const wrap = el("section", "sm-modal-section");
  wrap.appendChild(el("h3", "sm-modal-section-title", title));
  wrap.appendChild(body);
  return wrap;
}

function ulFromStrings(items: string[]): HTMLUListElement {
  const ul = el("ul", "sm-modal-list");
  for (const t of items) {
    ul.appendChild(el("li", undefined, t));
  }
  return ul;
}

export function closeStoryModal(): void {
  if (keyHandler) {
    document.removeEventListener("keydown", keyHandler);
    keyHandler = null;
  }
  loadAbort?.abort();
  loadAbort = null;
  overlayEl?.remove();
  overlayEl = null;
  document.body.classList.remove("sm-modal-open");
}

export function openStoryModal(opts: {
  card: Card;
  columnLabel: string;
  laneMilestone: Milestone;
  personaById: Map<string, Persona>;
}): void {
  closeStoryModal();
  const { card, columnLabel, laneMilestone, personaById } = opts;
  loadAbort = new AbortController();
  const signal = loadAbort.signal;

  document.body.classList.add("sm-modal-open");

  const overlay = el("div", "sm-modal-overlay");
  overlay.setAttribute("role", "presentation");
  overlay.addEventListener("click", (e) => {
    if (e.target === overlay) closeStoryModal();
  });
  overlayEl = overlay;

  const dialog = el("div", "sm-modal-dialog");
  dialog.setAttribute("role", "dialog");
  dialog.setAttribute("aria-modal", "true");
  dialog.setAttribute("aria-labelledby", "sm-modal-title");

  const top = el("div", "sm-modal-top");
  const title = el("h2", "sm-modal-title", card.title);
  title.id = "sm-modal-title";
  top.appendChild(title);

  if (card.userStoryId !== undefined) {
    const usText = card.userStoryId === null ? "—" : card.userStoryId;
    top.appendChild(el("p", "sm-modal-us-id", usText));
  }

  const closeBtn = el("button", "sm-modal-close", "×");
  closeBtn.type = "button";
  closeBtn.setAttribute("aria-label", "Close");
  closeBtn.addEventListener("click", () => closeStoryModal());
  top.appendChild(closeBtn);
  dialog.appendChild(top);

  if (card.subtitle) {
    dialog.appendChild(el("p", "sm-modal-subtitle", card.subtitle));
  }

  const ctx = el("div", "sm-modal-context");
  ctx.appendChild(el("span", "sm-modal-pill", columnLabel));
  if (card.userStoryId) {
    ctx.appendChild(el("span", "sm-modal-pill sm-modal-pill--us", card.userStoryId));
  }
  ctx.appendChild(el("span", "sm-modal-pill sm-modal-pill--milestone", laneMilestone.label));
  const st = el("span", `sm-status sm-status--${card.status} sm-modal-status`);
  st.textContent = cardStatusLabel(card.status);
  ctx.appendChild(st);
  if (card.personaIds?.length) {
    const row = el("span", "sm-modal-persona-row");
    for (const pid of card.personaIds) {
      row.appendChild(personaGlyph(personaById.get(pid), pid));
    }
    ctx.appendChild(row);
  }
  dialog.appendChild(ctx);

  const story = card.story;

  if (story?.asA || story?.iWant || story?.soThat) {
    const box = el("div", "sm-modal-userstory");
    if (story.asA) {
      const row = el("p", "sm-modal-us-line");
      row.appendChild(el("strong", undefined, "As "));
      row.appendChild(document.createTextNode(story.asA));
      box.appendChild(row);
    }
    if (story.iWant) {
      const row = el("p", "sm-modal-us-line");
      row.appendChild(el("strong", undefined, "I want "));
      row.appendChild(document.createTextNode(story.iWant));
      box.appendChild(row);
    }
    if (story.soThat) {
      const row = el("p", "sm-modal-us-line");
      row.appendChild(el("strong", undefined, "So that "));
      row.appendChild(document.createTextNode(story.soThat));
      box.appendChild(row);
    }
    dialog.appendChild(section("User story", box));
  }

  const taskRefs = (card.refs ?? []).filter((r) => /^T\d{3}$/.test(String(r.label).trim()));
  if (taskRefs.length) {
    const list = el("ul", "sm-modal-list");
    for (const r of taskRefs) {
      const li = el("li", "sm-modal-task-ref");
      const strong = el("strong", undefined, r.label);
      li.appendChild(strong);
      if (r.path?.trim()) {
        li.appendChild(document.createTextNode(" \u2014 "));
        li.appendChild(el("code", "sm-modal-ref-path", r.path));
      }
      list.appendChild(li);
    }
    const taskWrap = el("div");
    taskWrap.appendChild(
      el(
        "p",
        "sm-modal-muted",
        "Each T### cites exactly one US.x in task files. When you reprioritise, move this card\u2019s milestone (and column if the US changes epic) and update refs + specify/traceability.md in the same change set.",
      ),
    );
    taskWrap.appendChild(list);
    dialog.appendChild(section(`Technical tasks (${taskRefs.length})`, taskWrap));
  }

  if (story?.explanationMarkdown?.trim()) {
    const prose = el("div", "sm-prose");
    prose.innerHTML = renderMarkdown(story.explanationMarkdown);
    dialog.appendChild(section("Context & scope", prose));
  }

  if (story?.businessRules?.length) {
    dialog.appendChild(section("Business rules", ulFromStrings(story.businessRules)));
  }

  if (story?.definitionOfDone?.length) {
    dialog.appendChild(section("Definition of Done", ulFromStrings(story.definitionOfDone)));
  }

  if (story?.acceptanceCriteria?.length) {
    dialog.appendChild(section("Acceptance criteria", ulFromStrings(story.acceptanceCriteria)));
  }

  const traceRefs = (card.refs ?? []).filter(
    (r) => (r.path && !isSpecKitMarkdownRepoPath(r.path)) || Boolean(r.url),
  );
  if (traceRefs.length) {
    const list = el("ul", "sm-modal-refs");
    for (const r of traceRefs) {
      const li = el("li", "sm-modal-ref");
      if (r.url) {
        const a = el("a", "sm-ref-link");
        a.href = r.url;
        a.target = "_blank";
        a.rel = "noopener noreferrer";
        a.textContent = r.label;
        li.appendChild(a);
      } else {
        li.appendChild(el("span", "sm-modal-ref-label", `${r.label}: `));
        li.appendChild(el("code", "sm-modal-ref-path", r.path ?? ""));
      }
      list.appendChild(li);
    }
    dialog.appendChild(section("References (non–Spec-Kit .md)", list));
  }

  const specPaths = specMarkdownPathsForCard(card);
  const specHost = el("div", "sm-modal-spec-host");
  if (specPaths.length === 0) {
    specHost.appendChild(
      el(
        "p",
        "sm-modal-muted",
        "No Spec Kit Markdown files linked. Add `refs[].path` pointing at `specify/.../*.md` or `story.specPaths`, then run `npm run dev` / `npm run build` to sync `specify/` into the viewer.",
      ),
    );
  } else {
    const loading = el("p", "sm-modal-loading", "Loading Spec Kit documents…");
    specHost.appendChild(loading);

    const tabNav = el("div", "sm-modal-tabs");
    const tabPanels = el("div", "sm-modal-tabpanels");
    tabNav.hidden = true;
    tabPanels.hidden = true;
    specHost.appendChild(tabNav);
    specHost.appendChild(tabPanels);

    void (async () => {
      try {
        const results = await Promise.all(
          specPaths.map(async (path) => {
            const url = specKitFetchUrl(path);
            const res = await fetch(url, { signal, cache: "no-store" });
            const raw = res.ok ? await res.text() : `## HTTP error ${res.status}\n\n\`${url}\``;
            const html = renderMarkdown(raw);
            return { path, html };
          }),
        );
        if (signal.aborted) return;
        loading.remove();
        tabNav.hidden = false;
        tabPanels.hidden = false;

        const buttons: HTMLButtonElement[] = [];
        const panels: HTMLElement[] = [];

        const setActive = (i: number) => {
          buttons.forEach((b, j) => {
            b.classList.toggle("sm-modal-tab--active", j === i);
            b.setAttribute("aria-selected", j === i ? "true" : "false");
          });
          panels.forEach((p, j) => {
            p.hidden = j !== i;
          });
        };

        results.forEach((r, i) => {
          const tabId = `sm-tab-${i}`;
          const panelId = `sm-panel-${i}`;
          const btn = el("button", "sm-modal-tab", shortTabLabel(r.path)) as HTMLButtonElement;
          btn.type = "button";
          btn.id = tabId;
          btn.setAttribute("role", "tab");
          btn.setAttribute("aria-controls", panelId);
          btn.setAttribute("aria-selected", i === 0 ? "true" : "false");
          btn.addEventListener("click", () => setActive(i));
          buttons.push(btn);
          tabNav.appendChild(btn);

          const panel = el("div", "sm-modal-tabpanel");
          panel.id = panelId;
          panel.setAttribute("role", "tabpanel");
          panel.setAttribute("aria-labelledby", tabId);
          panel.hidden = i !== 0;
          const cap = el("div", "sm-modal-spec-caption");
          cap.appendChild(el("code", "sm-modal-spec-path", r.path));
          panel.appendChild(cap);
          const prose = el("div", "sm-prose");
          prose.innerHTML = r.html;
          panel.appendChild(prose);
          panels.push(panel);
          tabPanels.appendChild(panel);
        });
        if (buttons[0]) buttons[0].classList.add("sm-modal-tab--active");
      } catch (e) {
        if (signal.aborted) return;
        loading.textContent =
          e instanceof Error ? `Load failed: ${e.message}` : "Load failed.";
      }
    })();
  }

  dialog.appendChild(section("Spec Kit (Markdown, synced)", specHost));
  overlay.appendChild(dialog);
  document.body.appendChild(overlay);

  keyHandler = (ev: KeyboardEvent) => {
    if (ev.key === "Escape") {
      ev.preventDefault();
      closeStoryModal();
    }
  };
  document.addEventListener("keydown", keyHandler);

  closeBtn.focus();

  dialog.addEventListener("click", (ev) => {
    const t = ev.target as HTMLElement | null;
    if (t?.closest("a")) {
      ev.stopPropagation();
    }
  });
}

function shortTabLabel(repoPath: string): string {
  const base = repoPath.split("/").pop() ?? repoPath;
  return base.length > 28 ? `${base.slice(0, 25)}…` : base;
}
