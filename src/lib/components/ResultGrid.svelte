<script lang="ts">
  import { queryStore } from "../stores/query";
  import { connectionStore } from "../stores/connection";
  import { invoke } from "@tauri-apps/api/core";

  let activeTab = $derived(queryStore.tabs.find((t) => t.id === queryStore.activeTabId));
  let result = $derived(activeTab?.result);

  async function exportCSV() {
    if (!connectionStore.activeConnectionId || !activeTab?.sql) return;
    const csv = await invoke<string>("export_csv", {
      connectionId: connectionStore.activeConnectionId,
      sql: activeTab.sql,
    });
    const blob = new Blob([csv], { type: "text/csv" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "export.csv";
    a.click();
    URL.revokeObjectURL(url);
  }

  async function exportJSON() {
    if (!connectionStore.activeConnectionId || !activeTab?.sql) return;
    const json = await invoke<string>("export_json", {
      connectionId: connectionStore.activeConnectionId,
      sql: activeTab.sql,
    });
    const blob = new Blob([json], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "export.json";
    a.click();
    URL.revokeObjectURL(url);
  }
</script>

<div class="flex flex-col h-full bg-[#0c0c0c]">
  <div class="flex items-center justify-between px-3 py-1.5 bg-[#141414] border-b border-[#2a2a2a]">
    <span class="text-[11px] text-[#6b6b6b] uppercase tracking-wider">Results</span>
    <div class="flex items-center gap-2">
      {#if result}
        <span class="text-[11px] text-[#a0a0a0] font-mono mr-2">{result.row_count} rows · {result.execution_time_ms}ms</span>
        <button onclick={exportCSV} class="text-[10px] font-medium px-2 py-0.5 bg-[#1a1a1a] border border-[#2a2a2a] rounded text-[#a0a0a0] hover:text-[#e8e8e8] hover:border-[#333333]">CSV</button>
        <button onclick={exportJSON} class="text-[10px] font-medium px-2 py-0.5 bg-[#1a1a1a] border border-[#2a2a2a] rounded text-[#a0a0a0] hover:text-[#e8e8e8] hover:border-[#333333]">JSON</button>
      {/if}
    </div>
  </div>

  {#if activeTab?.status === "running"}
    <div class="flex-1 flex items-center justify-center">
      <div class="flex items-center gap-2 text-[#a0a0a0]">
        <div class="w-4 h-4 border-2 border-[#2a2a2a] border-t-[#00d4ff] rounded-full animate-spin"></div>
        <span class="text-[12px]">Executing...</span>
      </div>
    </div>
  {:else if activeTab?.status === "error"}
    <div class="flex-1 flex items-center justify-center p-4">
      <div class="text-[#ef4444] text-[12px] font-mono max-w-lg">{activeTab.error}</div>
    </div>
  {:else if result && result.columns.length > 0}
    <div class="flex-1 overflow-auto">
      <table class="w-full text-[12px]">
        <thead class="sticky top-0 bg-[#141414]">
          <tr>
            {#each result.columns as col}
              <th class="text-left px-3 py-1.5 text-[#a0a0a0] font-medium border-b border-[#2a2a2a] whitespace-nowrap">{col.name}</th>
            {/each}
          </tr>
        </thead>
        <tbody>
          {#each result.rows as row, i}
            <tr class="hover:bg-[rgba(0,212,255,0.05)] {i % 2 === 1 ? 'bg-[rgba(255,255,255,0.015)]' : ''}">
              {#each row as cell}
                <td class="px-3 py-1.5 text-[#e8e8e8] font-mono border-b border-[#1e1e1e] whitespace-nowrap max-w-xs truncate">
                  {#if cell === null}
                    <span class="text-[#6b6b6b]">NULL</span>
                  {:else}
                    {cell}
                  {/if}
                </td>
              {/each}
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {:else}
    <div class="flex-1 flex items-center justify-center">
      <p class="text-[12px] text-[#6b6b6b]">Run a query to see results</p>
    </div>
  {/if}
</div>
