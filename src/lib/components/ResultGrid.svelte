<script lang="ts">
  import { queryStore } from "../stores/query";

  let activeTab = $derived(queryStore.tabs.find((t) => t.id === queryStore.activeTabId));
  let result = $derived(activeTab?.result);
</script>

<div class="flex flex-col h-full bg-[#0c0c0c]">
  <div class="flex items-center justify-between px-3 py-1.5 bg-[#141414] border-b border-[#2a2a2a]">
    <span class="text-[11px] text-[#6b6b6b] uppercase tracking-wider">Results</span>
    {#if result}
      <span class="text-[11px] text-[#a0a0a0] font-mono">{result.row_count} rows · {result.execution_time_ms}ms</span>
    {/if}
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
