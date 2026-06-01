/// <reference types="vite/client" />

declare module "@backlog/*.json" {
  import type { StoryMapData } from "./types";
  const value: StoryMapData;
  export default value;
}
