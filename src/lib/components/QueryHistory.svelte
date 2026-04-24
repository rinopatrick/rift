<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { connectionStore } from "../stores/connection";
  import { queryStore } from "../stores/query";
  import type { QueryHistoryItem } from "../types";

  let history = $state<QueryHistoryItem[]>([]);
  let visible = $state(false);

  async function loadHistory() {
    if (!connectionStore.activeConnectionId) return;
    history = await invoke<QueryHistoryItem[]>("get_query_history", {
      connectionId: connectionStore.activeConnectionId,
    });
  }

  function useQuery(sql: string) {
    const activeTab = queryStore.tabs.find((t) => t.id === queryStore.activeTabId);
    if (activeTab) {
      queryStore.updateSql(activeTab.id, sql);
    }
    visible = false;
  }

  $effect(() => {
    if (connectionStore.activeConnectionId) {
      loadHistory();
    }
  });
</script>

<div class="relative">
  <button
    onclick={() => { visible = !visible; if (visible) loadHistory(); }}
    class="text-[11px] font-medium text-[#6b6b6b] hover:text-[#a0a0a0] px-2 py-1"
    title="Query History"
  >
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/>
    </svg>
  </button>

  {#if visible}
    <div class="absolute bottom-8 right-0 w-80 bg-[#141414] border border-[#2a2a2a] rounded-lg shadow-xl z-50 max-h-64 overflow-y-auto">
      <div class="flex items-center justify-between px-3 py-2 border-b border-[#2a2a2a]">
        <span class="text-[11px] font-medium text-[#a0a0a0]">Query History</span>
        <button onclick={() => visible = false} class="text-[#6b6b6b] hover:text-[#e8e8e8]">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
        </button>
      </div>
      {#if history.length > 0}
        <div class="divide-y divide-[#1e1e1e]">
          {#each history as item}
            <button
              onclick={() => useQuery(item.query)}
              class="w-full text-left px-3 py-2 text-[11px] font-mono text-[#a0a0a0] hover:bg-[#1f1f1f] hover:text-[#e8e8e8] truncate"
              title={item.query}
            >
              {item.query}
            </button>
          {/each}
        </div>
      {:else}
        <p class="text-[11px] text-[#6b6b6b] px-3 py-4 text-center">No history yet</p>
      {/if}
    </div>
  {/if}
</div>
