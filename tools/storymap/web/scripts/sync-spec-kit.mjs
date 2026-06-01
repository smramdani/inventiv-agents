/**
 * Copies repo `specify/` into public/spec-kit/specify/ so the viewer can
 * fetch Spec Kit markdown at /spec-kit/specify/... (dev + build + preview).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const webRoot = path.resolve(here, "..");
const repoRoot = path.resolve(webRoot, "..", "..", "..");
const src = path.join(repoRoot, "specify");
const dest = path.join(webRoot, "public", "spec-kit", "specify");

if (!fs.existsSync(src)) {
  console.warn("sync-spec-kit: no specify/ at repo root, skipping:", src);
  process.exit(0);
}

fs.mkdirSync(path.dirname(dest), { recursive: true });
fs.cpSync(src, dest, { recursive: true });
console.log("sync-spec-kit:", dest);
