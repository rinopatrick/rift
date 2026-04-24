/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        "rift-base": "#0c0c0c",
        "rift-surface": "#141414",
        "rift-elevated": "#1a1a1a",
        "rift-hover": "#1f1f1f",
        "rift-active": "#262626",
        "rift-cyan": "#00d4ff",
        "rift-blue": "#0ea5e9",
      },
      fontFamily: {
        sans: ["Inter", "-apple-system", "BlinkMacSystemFont", "Segoe UI", "Roboto", "sans-serif"],
        mono: ["JetBrains Mono", "Fira Code", "Source Code Pro", "Menlo", "Monaco", "monospace"],
      },
    },
  },
  plugins: [],
};
