<script lang="ts">
  import { queryStore } from "../stores/query";
  import { connectionStore } from "../stores/connection";
  import { schemaStore } from "../stores/schema";
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

  // Inline editing
  let editingCell = $state<{ row: number; col: number } | null>(null);
  let editValue = $state("");
  let updating = $state(false);

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

  function parseTableName(sql: string): string | null {
    const m = sql.match(/FROM\s+["']?(\w+)["']?/i);
    return m ? m[1] : null;
  }

  function getPrimaryKey(tableName: string): { schema: string; column: string } | null {
    for (const schema of schemaStore.schemas) {
      for (const table of schema.tables) {
        if (table.name === tableName) {
          const pk = table.columns.find((c) => c.is_primary_key);
          if (pk) return { schema: schema.name, column: pk.name };
        }
      }
    }
    return null;
  }

  function getPkValue(row: (string | null)[], tableName: string): string | null {
    const pk = getPrimaryKey(tableName);
    if (!pk || !result) return null;
    const colIndex = result.columns.findIndex((c) => c.name === pk.column);
    return colIndex >= 0 ? row[colIndex] : null;
  }

  async function commitEdit(rowIndex: number, colIndex: number) {
    if (!activeTab || !result || !connectionStore.activeConnectionId) return;
    const tableName = parseTableName(activeTab.sql);
    if (!tableName) {
      editingCell = null;
      return;
    }
    const pk = getPrimaryKey(tableName);
    const pkValue = getPkValue(result.rows[rowIndex], tableName);
    const colName = result.columns[colIndex].name;
    if (!pk || pkValue === null) {
      editingCell = null;
      return;
    }

    updating = true;
    try {
      const updateSql = `UPDATE "${pk.schema}"."${tableName}" SET "${colName}" = ${editValue === "null" || editValue === "" ? "NULL" : `'${editValue.replace(/'/g, "''")}'`} WHERE "${pk.column}" = '${pkValue.replace(/'/g, "''")}'`;
      await invoke("execute_sql", {
        connectionId: connectionStore.activeConnectionId,
        sql: updateSql,
        queryId: crypto.randomUUID(),
      });
      // Refresh
      await queryStore.executeQuery(connectionStore.activeConnectionId, activeTab.id);
    } catch (e) {
      console.error("Update failed:", e);
    } finally {
      updating = false;
      editingCell = null;
    }
  }

  function startEdit(rowIndex: number, colIndex: number, value: string | null) {
    const tableName = parseTableName(activeTab?.sql ?? "");
    if (!tableName || !getPrimaryKey(tableName)) return;
    editingCell = { row: rowIndex, col: colIndex };
    editValue = value ?? "";
  }

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

  {#if activeTab?.status === "running" || updating}
    <div class="flex-1 flex items-center justify-center">
      <div class="flex items-center gap-2 text-[#a0a0a0]">
        <div class="w-4 h-4 border-2 border-[#2a2a2a] border-t-[#00d4ff] rounded-full animate-spin"></div>
        <span class="text-[12px]">{updating ? "Saving..." : "Executing..."}</span>
      </div>
    </div>
  {:else if activeTab?.status === "error"}
    <div class="flex-1 flex items-center justify-center p-4">
      <div class="text-[#ef4444] text-[12px] font-mono max-w-lg">{activeTab.error}</div>
    </div>
  {:else if activeTab?.status === "cancelled"}
    <div class="flex-1 flex items-center justify-center p-4">
      <div class="text-[#a0a0a0] text-[12px] font-mono">Query cancelled by user</div>
    </div>
  {:else if result && result.columns.length > 0}
    {@const tableName = parseTableName(activeTab?.sql ?? "")}
    {@const editable = tableName && getPrimaryKey(tableName) !== null}
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
                {@const isEditing = editingCell?.row === absoluteIndex && editingCell?.col === j}
                <div class="px-3 text-[#e8e8e8] font-mono text-[12px] whitespace-nowrap truncate border-r border-[#1e1e1e] last:border-r-0 flex items-center flex-shrink-0 {editable ? 'cursor-pointer' : ''}"
                     style="min-width: 80px; width: {Math.max(80, result.columns[j].name.length * 9)}px;"
                     ondblclick={() => editable && startEdit(absoluteIndex, j, cell)}>
                  {#if isEditing}
                    <input
                      bind:value={editValue}
                      onkeydown={(e) => {
                        if (e.key === "Enter") commitEdit(absoluteIndex, j);
                        if (e.key === "Escape") editingCell = null;
                      }}
                      onblur={() => commitEdit(absoluteIndex, j)}
                      class="w-full bg-[#1a3a4a] text-[#e8e8e8] font-mono text-[12px] px-1 py-0.5 rounded outline-none border border-[#00d4ff]"
                      autofocus
                    />
                  {:else if cell === null}
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
    {#if editable}
      <div class="px-3 py-1 bg-[#141414] border-t border-[#2a2a2a] text-[10px] text-[#4a4a4a]">
        Double-click any cell to edit · Primary key required
      </div>
    {/if}
  {:else}
    <div class="flex-1 flex items-center justify-center">
      <p class="text-[12px] text-[#6b6b6b]">Run a query to see results</p>
    </div>
  {/if}
</div>
