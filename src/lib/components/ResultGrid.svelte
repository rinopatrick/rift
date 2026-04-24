<script lang="ts">
  import { queryStore } from "../stores/query";
  import { connectionStore } from "../stores/connection";
  import { invoke } from "@tauri-apps/api/core";

  let activeTab = $derived(queryStore.tabs.find((t) => t.id === queryStore.activeTabId));
  let result = $derived(activeTab?.result);

  // Virtualization
  const ROW_HEIGHT = 28;
  const BUFFER = 8;
  let scrollTop = $state(0);
  let containerHeight = $state(0);
  let viewportEl: HTMLDivElement;

  let totalHeight = $derived((result?.rows.length ?? 0) * ROW_HEIGHT);
  let visibleStart = $derived(Math.max(0, Math.floor(scrollTop / ROW_HEIGHT) - BUFFER));
  let visibleEnd = $derived(Math.min(
    result?.rows.length ?? 0,
    Math.ceil((scrollTop + containerHeight) / ROW_HEIGHT) + BUFFER
  ));
  let visibleRows = $derived(result?.rows.slice(visibleStart, visibleEnd) ?? []);
  let offsetY = $derived(visibleStart * ROW_HEIGHT);

  function onScroll() {
    if (!viewportEl) return;
    scrollTop = viewportEl.scrollTop;
  }

  function onResize() {
    if (!viewportEl) return;
    containerHeight = viewportEl.clientHeight;
  }

  $effect(() => {
    if (viewportEl) {
      onResize();
      const ro = new ResizeObserver(onResize);
      ro.observe(viewportEl);
      return () => ro.disconnect();
    }
  });

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
  <div class="flex items-center justify-between px-3 py-1.5 bg-[#141414] border-b border-[#2a2a2a] shrink-0">
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
    <div bind:this={viewportEl} onscroll={onScroll} class="flex-1 overflow-auto relative">
      <div style="height: {totalHeight}px; position: relative;">
        <!-- Sticky Header -->
        <div class="sticky top-0 z-20 bg-[#141414] border-b border-[#2a2a2a]">
          <div class="flex">
            {#each result.columns as col}
              <div class="text-left px-3 py-1.5 text-[#a0a0a0] font-medium text-[12px] whitespace-nowrap border-r border-[#2a2a2a] last:border-r-0 flex-shrink-0" style="min-width: 80px; width: {Math.max(80, col.name.length * 9)}px;">
                {col.name}
              </div>
            {/each}
          </div>
        </div>

        <!-- Virtual Rows -->
        <div style="transform: translateY({offsetY}px);">
          {#each visibleRows as row, i (visibleStart + i)}
            {@const absoluteIndex = visibleStart + i}
            <div class="flex hover:bg-[rgba(0,212,255,0.04)] {absoluteIndex % 2 === 1 ? 'bg-[rgba(255,255,255,0.015)]' : ''}"
                 style="height: {ROW_HEIGHT}px;">
              {#each row as cell, j}
                <div class="px-3 text-[#e8e8e8] font-mono text-[12px] whitespace-nowrap truncate border-r border-[#1e1e1e] last:border-r-0 flex items-center flex-shrink-0"
                     style="min-width: 80px; width: {Math.max(80, result.columns[j].name.length * 9)}px;">
                  {#if cell === null}
                    <span class="text-[#6b6b6b]">NULL</span>
                  {:else}
                    {cell}
                  {/if}
                </div>
              {/each}
            </div>
          {/each}
        </div>
      </div>
    </div>
  {:else}
    <div class="flex-1 flex items-center justify-center">
      <p class="text-[12px] text-[#6b6b6b]">Run a query to see results</p>
    </div>
  {/if}
</div>
