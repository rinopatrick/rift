<script lang="ts">
  import { connectionStore } from "../stores/connection";
  import { queryStore } from "../stores/query";
  import QueryHistory from "./QueryHistory.svelte";

  let activeConn = $derived(
    connectionStore.connections.find((c) => c.id === connectionStore.activeConnectionId)
  );
  let activeTab = $derived(queryStore.tabs.find((t) => t.id === queryStore.activeTabId));
  let activeResult = $derived(activeTab ? queryStore.getActiveResult(activeTab.id) : null);
</script>

<div class="h-6 bg-[#0c0c0c] border-t border-[#1e1e1e] flex items-center px-3 text-[11px] font-mono">
  <div class="flex items-center gap-2 text-[#6b6b6b]">
    {#if activeConn}
      <span class="w-1.5 h-1.5 rounded-full bg-[#00d4ff]"></span>
      <span>{activeConn.name}</span>
      <span class="text-[#333333]">|</span>
      <span>{activeConn.database}</span>
    {:else}
      <span class="w-1.5 h-1.5 rounded-full bg-[#6b6b6b]"></span>
      <span>Disconnected</span>
    {/if}
  </div>

  <div class="flex-1"></div>

  <div class="flex items-center gap-3 text-[#a0a0a0]">
    {#if activeResult}
      <span>{activeResult.row_count} rows</span>
      <span>{activeResult.execution_time_ms}ms</span>
    {/if}
    {#if activeTab?.status === "running"}
      <span class="text-[#00d4ff]">Executing...</span>
    {:else if activeTab?.status === "cancelled"}
      <span class="text-[#ff8c42]">Cancelled</span>
    {/if}
    <QueryHistory />
  </div>
</div>
