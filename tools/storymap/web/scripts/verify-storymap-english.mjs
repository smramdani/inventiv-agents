/**
 * Fails if storymap backlog, docs, or viewer sources contain likely non-English
 * (French accents / common French tokens). Application i18n lives elsewhere.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const webRoot = path.resolve(here, "..");
const storymapRoot = path.resolve(webRoot, "..");

/** Latin letters commonly used in French / Romance text, rare in EN-only OSS docs. */
const ACCENTED_LATIN = /[àâäéèêëïîôùûüçœæÀÂÄÉÈÊËÏÎÔÙÛÜÇŒÆ]/;

/**
 * Whole-word hints (French); tuned to reduce false positives in English prose.
 */
const FRENCH_HINT = new RegExp(
  String.raw`\b(` +
    [
      "les",
      "des",
      "une",
      "pour",
      "avec",
      "dans",
      "être",
      "était",
      "été",
      "sera",
      "créer",
      "données",
      "équipe",
      "voici",
      "échec",
      "chargement",
      "périmètre",
      "règles",
      "métier",
      "références",
      "ouvrir",
      "détail",
      "très",
      "aussi",
      "notre",
      "votre",
      "leur",
      "celui",
      "cette",
      "éphémère",
      "côté",
      "parcours",
      "livré",
      "réalisés",
      "lorsque",
      "lorsqu",
      "afin",
    ].join("|") +
    String.raw`)\b`,
  "i",
);

/** @type {{ file: string; line: number | null; reason: string; excerpt: string }[]} */
const findings = [];

function record(file, line, reason, excerpt) {
  findings.push({ file, line, reason, excerpt: excerpt.trim().slice(0, 120) });
}

function checkText(text, file, lineNo) {
  if (ACCENTED_LATIN.test(text)) {
    record(file, lineNo, "Accented Latin (use ASCII English in repo text)", text);
  }
  const m = text.match(FRENCH_HINT);
  if (m) {
    record(file, lineNo, `French/common non-EN token "${m[1]}"`, text);
  }
}

function scanLines(file, content) {
  const lines = content.split(/\r?\n/);
  lines.forEach((line, i) => {
    if (line.includes("//") || line.includes("*")) {
      // Still scan comments — repo policy is English-only everywhere in storymap sources.
    }
    checkText(line, file, i + 1);
  });
}

function collectJsonStrings(val, out) {
  if (typeof val === "string") out.push(val);
  else if (Array.isArray(val)) val.forEach((v) => collectJsonStrings(v, out));
  else if (val && typeof val === "object") Object.values(val).forEach((v) => collectJsonStrings(v, out));
}

function scanJsonFile(file) {
  const raw = fs.readFileSync(file, "utf8");
  let data;
  try {
    data = JSON.parse(raw);
  } catch (e) {
    record(file, 1, `Invalid JSON: ${e instanceof Error ? e.message : String(e)}`, raw.slice(0, 80));
    return;
  }
  const strings = [];
  collectJsonStrings(data, strings);
  strings.forEach((s, i) => {
    const issues = [];
    if (ACCENTED_LATIN.test(s)) issues.push("accented Latin");
    const fm = s.match(FRENCH_HINT);
    if (fm) issues.push(`token "${fm[1]}"`);
    if (issues.length) {
      const excerpt = s.length > 100 ? `${s.slice(0, 100)}…` : s;
      record(file, null, `JSON string value: ${issues.join("; ")}`, excerpt);
    }
  });
}

function walk(dir, skipNames, visit) {
  if (!fs.existsSync(dir)) return;
  for (const ent of fs.readdirSync(dir, { withFileTypes: true })) {
    if (skipNames.has(ent.name)) continue;
    const p = path.join(dir, ent.name);
    if (ent.isDirectory()) walk(p, skipNames, visit);
    else visit(p);
  }
}

// --- backlog JSON ---
const backlogDir = path.join(storymapRoot, "backlog");
if (fs.existsSync(backlogDir)) {
  for (const name of fs.readdirSync(backlogDir)) {
    if (!name.endsWith(".json")) continue;
    scanJsonFile(path.join(backlogDir, name));
  }
}

// --- storymap docs + README ---
for (const rel of ["README.md", "docs/format.md", "docs/renderer.md"]) {
  const file = path.join(storymapRoot, rel);
  if (fs.existsSync(file)) {
    scanLines(file, fs.readFileSync(file, "utf8"));
  }
}

// --- viewer sources (no node_modules / dist) ---
const skip = new Set(["node_modules", "dist", "public"]);
walk(
  path.join(webRoot, "src"),
  skip,
  (p) => {
    if (p.endsWith(".ts")) scanLines(p, fs.readFileSync(p, "utf8"));
  },
);

const indexHtml = path.join(webRoot, "index.html");
if (fs.existsSync(indexHtml)) {
  scanLines(indexHtml, fs.readFileSync(indexHtml, "utf8"));
}

if (findings.length) {
  console.error("verify-storymap-english: FAILED\n");
  for (const f of findings) {
    const loc = f.line == null ? `${f.file} (JSON)` : `${f.file}:${f.line}`;
    console.error(`${loc}\n  ${f.reason}\n  ${f.excerpt}\n`);
  }
  console.error(`Total issues: ${findings.length}`);
  process.exit(1);
}

console.log("verify-storymap-english: OK (backlog JSON, tools/storymap docs, web/src, index.html)");
