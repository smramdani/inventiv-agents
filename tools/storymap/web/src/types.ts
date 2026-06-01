/** Backlog JSON contract (v2). See ../docs/format.md */

export type CardStatus = "planned" | "in_progress" | "shipped" | "deferred";

export interface StoryMapMeta {
  title: string;
  formatVersion: 2;
  description?: string;
}

export interface Persona {
  id: string;
  label: string;
  /** Optional pictogram shown on chips and on cards (any single grapheme cluster, e.g. emoji). */
  emoji?: string;
  description?: string;
}

export interface Column {
  id: string;
  label: string;
  order: number;
  description?: string;
  /** Optional epic id — matches `specify/spec.md` §2 (e.g. E-ONB). Shown on the board header. */
  epicId?: string;
}

/** Swimlane = milestone (planning), not Constitution XV software release. */
export interface Milestone {
  id: string;
  label: string;
  order: number;
  color?: string;
}

export interface CardRef {
  label: string;
  path?: string;
  url?: string;
}

/** Rich user-story + acceptance; Spec Kit `.md` under `specify/` loaded at click (see sync-spec-kit). */
export interface CardStory {
  /** Extra Spec Kit markdown paths (repo-relative, must stay under `specify/`). Merged with `refs[].path` for `*.md`. */
  specPaths?: string[];
  asA?: string;
  iWant?: string;
  soThat?: string;
  /** Markdown block (context, scope notes). */
  explanationMarkdown?: string;
  businessRules?: string[];
  definitionOfDone?: string[];
  acceptanceCriteria?: string[];
}

export interface Card {
  id: string;
  columnId: string;
  title: string;
  subtitle?: string;
  milestoneId: string;
  /** `US.1`–`US.15` from `specify/spec.md` §7 (§7.0–7.15). One US is bound to a single milestone, so each `userStoryId` appears on at most one card per backlog file. */
  userStoryId?: string | null;
  status: CardStatus;
  personaIds?: string[];
  refs?: CardRef[];
  story?: CardStory;
}

export interface StoryMapData {
  meta: StoryMapMeta;
  personas?: Persona[];
  columns: Column[];
  milestones: Milestone[];
  cards: Card[];
}
