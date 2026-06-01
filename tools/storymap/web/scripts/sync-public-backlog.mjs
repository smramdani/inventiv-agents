/**
 * Copies ../backlog/*.json into public/backlog/ so the viewer can fetch
 * them at /backlog/<name> (dev, preview, and static dist).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const webRoot = path.resolve(here, "..");
const backlogDir = path.resolve(webRoot, "..", "backlog");
const destDir = path.join(webRoot, "public", "backlog");

fs.mkdirSync(destDir, { recursive: true });
for (const name of fs.readdirSync(backlogDir)) {
  if (!name.endsWith(".json")) continue;
  fs.copyFileSync(path.join(backlogDir, name), path.join(destDir, name));
}
