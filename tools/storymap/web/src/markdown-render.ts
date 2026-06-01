import { marked } from "marked";

marked.use({ gfm: true });

export function renderMarkdown(source: string): string {
  const out = marked.parse(source, { async: false });
  if (typeof out !== "string") {
    throw new Error("marked.parse expected sync string");
  }
  return out;
}
