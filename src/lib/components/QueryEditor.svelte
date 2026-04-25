<script lang="ts">
  import { queryStore } from "../stores/query";
  import { connectionStore } from "../stores/connection";
  import { schemaStore } from "../stores/schema";
  import CodeMirrorEditor from "./CodeMirrorEditor.svelte";
  import type { Completion } from "@codemirror/autocomplete";
  import { invoke } from "@tauri-apps/api/core";

  let activeTab = $derived(queryStore.tabs.find((t) => t.id === queryStore.activeTabId));

  let completions = $derived<Completion[]>(() => {
    const items: Completion[] = [];
    
    // SQL keywords
    const keywords = [
      "SELECT", "FROM", "WHERE", "INSERT", "UPDATE", "DELETE", "CREATE", "DROP",
      "ALTER", "TABLE", "INDEX", "VIEW", "JOIN", "LEFT", "RIGHT", "INNER", "OUTER",
      "ON", "GROUP", "BY", "ORDER", "HAVING", "LIMIT", "OFFSET", "UNION", "ALL",
      "DISTINCT", "AS", "AND", "OR", "NOT", "NULL", "IS", "IN", "EXISTS", "BETWEEN",
      "LIKE", "ILIKE", "CASE", "WHEN", "THEN", "ELSE", "END", "CAST", "COALESCE",
      "COUNT", "SUM", "AVG", "MIN", "MAX", "RETURNING", "VALUES", "SET", "INTO",
    ];
    keywords.forEach(kw => {
      items.push({ label: kw, type: "keyword", detail: "SQL" });
    });

    // Schema items
    schemaStore.schemas.forEach(schema => {
      items.push({ label: schema.name, type: "namespace", detail: "schema" });
      schema.tables.forEach(table => {
        const qualifiedName = `${schema.name}.${table.name}`;
        items.push({ label: table.name, type: "class", detail: qualifiedName });
        items.push({ label: qualifiedName, type: "class", detail: "table" });
        table.columns.forEach(col => {
          items.push({ label: col.name, type: "property", detail: `${qualifiedName} (${col.dataType})` });
        });
      });
    });

    return items;
  });

  function handleRun() {
    if (connectionStore.activeConnectionId && activeTab) {
      queryStore.executeQuery(connectionStore.activeConnectionId, activeTab.id);
    }
  }

  function handleRunNewTab() {
    if (connectionStore.activeConnectionId && activeTab) {
      const newTab = queryStore.addTab();
      queryStore.updateSql(newTab.id, activeTab.sql);
      queryStore.executeQuery(connectionStore.activeConnectionId, newTab.id);
    }
  }

  function handleCancel() {
    if (activeTab) {
      queryStore.cancelQuery(activeTab.id);
    }
  }

  async function handleExplain() {
    if (!connectionStore.activeConnectionId || !activeTab) return;
    try {
      queryStore.setExplainLoading(activeTab.id, true);
      const data = await invoke("explain_query", {
        connectionId: connectionStore.activeConnectionId,
        sql: activeTab.sql,
      });
      queryStore.setExplainData(activeTab.id, data);
    } catch (err) {
      queryStore.setExplainError(activeTab.id, String(err));
    } finally {
      queryStore.setExplainLoading(activeTab.id, false);
    }
  }

  async function handleProfile() {
    if (!connectionStore.activeConnectionId || !activeTab) return;
    try {
      queryStore.setProfileLoading(activeTab.id, true);
      const data = await invoke("profile_query", {
        connectionId: connectionStore.activeConnectionId,
        sql: activeTab.sql,
      });
      queryStore.setProfileData(activeTab.id, data);
    } catch (err) {
      queryStore.setProfileError(activeTab.id, String(err));
    } finally {
      queryStore.setProfileLoading(activeTab.id, false);
    }
  }

  function handleChange(value: string) {
    if (activeTab) {
      queryStore.updateSql(activeTab.id, value);
    }
  }
</script>

<div class="flex flex-col h-full">
  <div class="flex items-center justify-between px-3 py-1.5 bg-[#141414] border-b border-[#2a2a2a]">
    <span class="text-[11px] text-[#6b6b6b] uppercase tracking-wider">Query Editor</span>
    <div class="flex items-center gap-2">
      <span class="text-[10px] text-[#4a4a4a] font-mono hidden sm:inline">Ctrl+↵ Run · Ctrl+Shift+↵ New Tab</span>
      {#if activeTab?.status === "running"}
        <button
          onclick={handleCancel}
          class="flex items-center gap-1.5 px-3 py-1 text-[11px] font-medium bg-[#ff4444] text-white rounded hover:brightness-110"
        >
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="6" y="6" width="12" height="12" rx="2"/></svg>
          Cancel
        </button>
      {:else}
        <button
          onclick={handleRun}
          disabled={!connectionStore.activeConnectionId}
          class="flex items-center gap-1.5 px-3 py-1 text-[11px] font-medium bg-[#00d4ff] text-black rounded hover:brightness-110 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M5 12h14"/><path d="m12 5 7 7-7 7"/></svg>
          Run
        </button>
        <button
          onclick={handleExplain}
          disabled={!connectionStore.activeConnectionId}
          class="flex items-center gap-1.5 px-3 py-1 text-[11px] font-medium bg-[#1a1a1a] border border-[#2a2a2a] text-[#a0a0a0] rounded hover:text-[#e8e8e8] hover:border-[#333333] disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>
          Explain
        </button>
        <button
          onclick={handleProfile}
          disabled={!connectionStore.activeConnectionId}
          class="flex items-center gap-1.5 px-3 py-1 text-[11px] font-medium bg-[#1a1a1a] border border-[#2a2a2a] text-[#a0a0a0] rounded hover:text-[#e8e8e8] hover:border-[#333333] disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 20v-6"/><path d="M6 20V10"/><path d="M18 20V4"/></svg>
          Profile
        </button>
      {/if}
    </div>
  </div>
  
  {#if activeTab}
    <CodeMirrorEditor
      value={activeTab.sql}
      onChange={handleChange}
      onRun={handleRun}
      onRunNewTab={handleRunNewTab}
      completions={completions()}
      disabled={!connectionStore.activeConnectionId}
    />
  {:else}
    <div class="flex-1 flex items-center justify-center bg-[#0c0c0c]">
      <p class="text-sm text-[#4a4a4a]">Open a connection to start writing queries</p>
    </div>
  {/if}
</div>
