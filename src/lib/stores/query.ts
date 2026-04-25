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

    const queryId = crypto.randomUUID();

    tabs = tabs.map((t) =>
      t.id === tabId ? { ...t, status: "running" as const, result: null, error: undefined, queryId } : t
    );

    try {
      const result = await invoke<QueryResult>("execute_sql", {
        connectionId,
        sql: tab.sql,
        queryId,
      });
      tabs = tabs.map((t) =>
        t.id === tabId ? { ...t, status: "idle" as const, result, queryId: undefined } : t
      );
      // Save to history
      await invoke("save_query_history", { connectionId, query: tab.sql });
    } catch (err) {
      const msg = String(err);
      const isCancelled = msg.includes("cancelled") || msg.includes("Cancel");
      tabs = tabs.map((t) =>
        t.id === tabId
          ? {
              ...t,
              status: isCancelled ? ("cancelled" as const) : ("error" as const),
              error: msg,
              queryId: undefined,
            }
          : t
      );
    }
  }

  async function cancelQuery(tabId: string) {
    const tab = tabs.find((t) => t.id === tabId);
    if (!tab?.queryId) return;

    try {
      await invoke("cancel_query", { queryId: tab.queryId });
    } catch (err) {
      // Ignore cancel errors
    }
  }

  function setExplainLoading(tabId: string, loading: boolean) {
    tabs = tabs.map((t) => t.id === tabId ? { ...t, explainLoading: loading } : t);
  }

  function setExplainData(tabId: string, data: any) {
    tabs = tabs.map((t) => t.id === tabId ? { ...t, explainData: data, explainError: undefined } : t);
  }

  function setExplainError(tabId: string, error: string) {
    tabs = tabs.map((t) => t.id === tabId ? { ...t, explainError: error } : t);
  }

  return {
    get tabs() { return tabs; },
    get activeTabId() { return activeTabId; },
    set activeTabId(id: string) { activeTabId = id; },
    addTab,
    removeTab,
    updateSql,
    executeQuery,
    cancelQuery,
    setExplainLoading,
    setExplainData,
    setExplainError,
  };
}

export const queryStore = createQueryStore();
