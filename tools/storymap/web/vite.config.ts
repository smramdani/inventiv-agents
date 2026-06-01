import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

const rootDir = fileURLToPath(new URL(".", import.meta.url));

export default defineConfig({
  root: rootDir,
  publicDir: path.resolve(rootDir, "public"),
  server: {
    port: 5190,
    host: "127.0.0.1",
    strictPort: false,
    fs: {
      allow: [path.resolve(rootDir, "..")],
    },
  },
  resolve: {
    alias: {
      "@backlog": path.resolve(rootDir, "../backlog"),
    },
  },
});
