import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

export default defineConfig(async () => ({
  plugins: [svelte()],
  clearScreen: false,
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          codemirror: [
            "@codemirror/view",
            "@codemirror/state",
            "@codemirror/language",
            "@codemirror/lang-sql",
            "@codemirror/autocomplete",
            "@codemirror/search",
            "@codemirror/lint",
            "@codemirror/commands",
            "@lezer/highlight",
          ],
        },
      },
    },
  },
  server: {
    port: 1420,
    strictPort: true,
    watch: { ignored: ["**/src-tauri/**"] },
  },
}));
