import type { Card } from "./types";

const norm = (p: string) => p.trim().replace(/\\/g, "/");

/** Repo-relative paths to Spec Kit markdown served after `sync-spec-kit` (under `specify/` only). */
export function isSpecKitMarkdownRepoPath(raw: string): boolean {
  const p = norm(raw);
  if (!p || p.includes("..")) return false;
  return p.startsWith("specify/") && p.endsWith(".md");
}

/** URL path under Vite `public/spec-kit/`. */
export function specKitFetchUrl(repoRelativePath: string): string {
  const p = norm(repoRelativePath);
  if (!isSpecKitMarkdownRepoPath(p)) {
    throw new Error(`Not a fetchable Spec Kit markdown path: ${repoRelativePath}`);
  }
  return `/spec-kit/${p.split("/").map(encodeURIComponent).join("/")}`;
}

/** Merges `story.specPaths` with `refs[].path` values under `specify/` ending in `.md` (deduped, stable order). */
export function specMarkdownPathsForCard(card: Card): string[] {
  const fromStory = card.story?.specPaths ?? [];
  const fromRefs = (card.refs ?? [])
    .map((r) => r.path)
    .filter((path): path is string => typeof path === "string" && isSpecKitMarkdownRepoPath(path));
  const seen = new Set<string>();
  const out: string[] = [];
  for (const p of [...fromStory, ...fromRefs]) {
    const n = norm(p);
    if (!seen.has(n)) {
      seen.add(n);
      out.push(n);
    }
  }
  return out;
}
