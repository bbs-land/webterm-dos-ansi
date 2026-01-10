import { defineConfig } from "npm:vite@^5";

export default defineConfig({
  build: {
    lib: {
      entry: "./js/index.js",
      name: "WebTermDosAnsi",
      fileName: "webterm-dos-ansi",
      formats: ["es"],
    },
    outDir: "dist",
    target: "esnext",
  },
  server: {
    port: 5173,
  },
});
