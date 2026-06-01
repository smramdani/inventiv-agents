import type { StoryMapData } from "./types";

function isRecord(x: unknown): x is Record<string, unknown> {
  return typeof x === "object" && x !== null && !Array.isArray(x);
}

/** Minimal structural checks + referential integrity for columns / milestones. */
export function assertStoryMapData(raw: unknown): asserts raw is StoryMapData {
  if (!isRecord(raw)) throw new Error("Backlog root must be an object");
  const meta = raw.meta;
  if (!isRecord(meta)) throw new Error('Missing "meta" object');
  if (typeof meta.title !== "string") throw new Error('meta.title must be a string');
  if (meta.formatVersion !== 2) throw new Error('meta.formatVersion must be 2 (use milestones + milestoneId; see docs/format.md)');

  if (!Array.isArray(raw.columns) || raw.columns.length === 0) {
    throw new Error('"columns" must be a non-empty array');
  }
  if (!Array.isArray(raw.milestones) || raw.milestones.length === 0) {
    throw new Error('"milestones" must be a non-empty array');
  }
  if (!Array.isArray(raw.cards)) throw new Error('"cards" must be an array');

  const columnIds = new Set<string>();
  for (const c of raw.columns) {
    if (!isRecord(c) || typeof c.id !== "string") throw new Error("Invalid column entry");
    columnIds.add(c.id);
  }
  const milestoneIds = new Set<string>();
  for (const m of raw.milestones) {
    if (!isRecord(m) || typeof m.id !== "string") throw new Error("Invalid milestone entry");
    milestoneIds.add(m.id);
  }
  const personaIds = new Set<string>();
  if (Array.isArray(raw.personas)) {
    for (const p of raw.personas) {
      if (!isRecord(p) || typeof p.id !== "string") throw new Error("Invalid persona entry");
      personaIds.add(p.id);
    }
  }

  const statuses = new Set(["planned", "in_progress", "shipped", "deferred"]);
  const userStoryToMilestone = new Map<string, { milestoneId: string; cardLabel: string }>();
  for (const card of raw.cards) {
    if (!isRecord(card)) throw new Error("Invalid card entry");
    const cardLabel = typeof card.id === "string" ? card.id : "(missing id)";
    if (typeof card.columnId !== "string" || !columnIds.has(card.columnId)) {
      throw new Error(`Card "${cardLabel}": unknown columnId "${String(card.columnId)}"`);
    }
    if (typeof card.milestoneId !== "string" || !milestoneIds.has(card.milestoneId)) {
      throw new Error(`Card "${cardLabel}": unknown milestoneId "${String(card.milestoneId)}"`);
    }
    if ("userStoryId" in card && card.userStoryId !== null && typeof card.userStoryId !== "string") {
      throw new Error(`Card "${cardLabel}": userStoryId must be string or null`);
    }
    if (typeof card.status !== "string" || !statuses.has(card.status)) {
      throw new Error(`Card "${cardLabel}": invalid status`);
    }
    if (Array.isArray(card.personaIds)) {
      for (const pid of card.personaIds) {
        if (typeof pid !== "string") {
          throw new Error(`Card "${cardLabel}": personaIds must be strings`);
        }
        if (!personaIds.has(pid)) {
          throw new Error(
            `Card "${cardLabel}": personaIds references unknown persona "${pid}". Add it to top-level "personas" or remove it from this card. Personas must mirror the "As a …" wording (see specify/spec.md §7).`,
          );
        }
      }
    }
    if (typeof card.userStoryId === "string" && card.userStoryId.length > 0) {
      const prev = userStoryToMilestone.get(card.userStoryId);
      if (prev !== undefined) {
        if (prev.milestoneId !== card.milestoneId) {
          throw new Error(
            `Card "${cardLabel}": userStoryId "${card.userStoryId}" already placed on milestone "${prev.milestoneId}" (card "${prev.cardLabel}") and cannot also be on "${String(card.milestoneId)}". A user story belongs to exactly one milestone — split it into a new US.x if you need a different milestone (see specify/spec.md §7 / specify/traceability.md §1.1).`,
          );
        }
        throw new Error(
          `Card "${cardLabel}": duplicate user story placement for userStoryId "${card.userStoryId}" (also card "${prev.cardLabel}"). Use exactly one card per user story; list multiple **T###** in refs.`,
        );
      }
      userStoryToMilestone.set(card.userStoryId, { milestoneId: String(card.milestoneId), cardLabel });
    }
  }
}
