import inventivagents from "@backlog/inventivagents.json";
import { renderError, renderStoryMap } from "./render";
import type { StoryMapData } from "./types";
import { assertStoryMapData } from "./validate";
import "./styles.css";

/** Basename only, no path segments (served from /backlog/ after sync). */
const BACKLOG_FILE_RE = /^[a-zA-Z0-9][a-zA-Z0-9_.-]*\.json$/;

function backlogFileFromSearch(): string | null {
  const params = new URLSearchParams(window.location.search);
  const raw = params.get("file") ?? params.get("backlog");
  if (!raw || raw.length > 128) return null;
  if (!BACKLOG_FILE_RE.test(raw)) return null;
  return raw;
}

async function loadData(): Promise<StoryMapData> {
  const file = backlogFileFromSearch();
  if (file) {
    const res = await fetch(`/backlog/${encodeURIComponent(file)}`, { cache: "no-store" });
    if (!res.ok) {
      throw new Error(`Cannot load /backlog/${file} (HTTP ${res.status}). Run dev/build so public/backlog is synced.`);
    }
    const json: unknown = await res.json();
    assertStoryMapData(json);
    return json;
  }
  assertStoryMapData(inventivagents);
  return inventivagents;
}

const app = document.querySelector("#app");
if (!app) {
  throw new Error("#app missing");
}

void loadData()
  .then((data) => {
    renderStoryMap(app as HTMLElement, data);
  })
  .catch((err: unknown) => {
    const message = err instanceof Error ? err.message : String(err);
    renderError(app as HTMLElement, message);
  });
