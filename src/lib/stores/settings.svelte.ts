import { invoke } from "@tauri-apps/api/core";

export interface AppSettings {
  editorFontSize: number;
  resultFontSize: number;
  theme: "dark" | "light" | "system";
  showLineNumbers: boolean;
  queryTimeout: number;
}

const DEFAULTS: AppSettings = {
  editorFontSize: 14,
  resultFontSize: 12,
  theme: "dark",
  showLineNumbers: true,
  queryTimeout: 30,
};

export function createSettingsStore() {
  let settings = $state<AppSettings>({ ...DEFAULTS });
  let loaded = $state(false);

  async function load() {
    try {
      const pairs = await invoke<[string, string][]>("get_settings");
      const map = new Map(pairs);
      settings = {
        editorFontSize: parseInt(map.get("editor_font_size") ?? "14", 10),
        resultFontSize: parseInt(map.get("result_font_size") ?? "12", 10),
        theme: (map.get("theme") as AppSettings["theme"]) ?? "dark",
        showLineNumbers: map.get("show_line_numbers") !== "false",
        queryTimeout: parseInt(map.get("query_timeout") ?? "30", 10),
      };
    } catch (err) {
      console.error("Failed to load settings:", err);
    }
    loaded = true;
  }

  async function save(partial: Partial<AppSettings>) {
    settings = { ...settings, ...partial };
    try {
      const pairs: [string, string][] = [
        ["editor_font_size", String(settings.editorFontSize)],
        ["result_font_size", String(settings.resultFontSize)],
        ["theme", settings.theme],
        ["show_line_numbers", String(settings.showLineNumbers)],
        ["query_timeout", String(settings.queryTimeout)],
      ];
      for (const [key, value] of pairs) {
        await invoke("save_setting", { key, value });
      }
    } catch (err) {
      console.error("Failed to save settings:", err);
    }
  }

  return {
    get settings() { return settings; },
    get loaded() { return loaded; },
    load,
    save,
  };
}

export const settingsStore = createSettingsStore();
