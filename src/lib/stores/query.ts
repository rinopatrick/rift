import { invoke } from "@tauri-apps/api/core";
import type { QueryResult, QueryTab } from "../types";

export function createQueryStore() {
  let tabs = $state<QueryTab[]>([
    { id: "1", name: "Query 1", sql: "", result: null, status: "idle" },
  ]);
  let activeTabId = $state<string>("1");

  function addTab() {
    const id = crypto.randomUUID();
    tabs = [...tabs, { id, name: `Query ${tabs.length + 1}`, sql: "", result: null, status: "idle" }];
    activeTabId = id;
  }

  function removeTab(id: string) {
    if (tabs.length <= 1) return;
    tabs = tabs.filter((t) => t.id !== id);
    if (activeTabId === id) {
      activeTabId = tabs[tabs.length - 1].id;
    }
  }

  function updateSql(id: string, sql: string) {
    tabs = tabs.map((t) => (t.id === id ? { ...t, sql } : t));
  }

  async function executeQuery(connectionId: string, tabId: string) {
    const tab = tabs.find((t) => t.id === tabId);
    if (!tab || !tab.sql.trim()) return;

    tabs = tabs.map((t) =>
      t.id === tabId ? { ...t, status: "running" as const, result: null, error: undefined } : t
    );

    try {
      const result = await invoke<QueryResult>("execute_sql", {
        connectionId,
        sql: tab.sql,
      });
      tabs = tabs.map((t) =>
        t.id === tabId ? { ...t, status: "idle" as const, result } : t
      );
      // Save to history
      await invoke("save_query_history", { connectionId, query: tab.sql });
    } catch (err) {
      tabs = tabs.map((t) =>
        t.id === tabId ? { ...t, status: "error" as const, error: String(err) } : t
      );
    }
  }

  return {
    get tabs() { return tabs; },
    get activeTabId() { return activeTabId; },
    set activeTabId(id: string) { activeTabId = id; },
    addTab,
    removeTab,
    updateSql,
    executeQuery,
  };
}

export const queryStore = createQueryStore();
