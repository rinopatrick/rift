<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { connectionStore } from "../stores/connection";
  import { schemaStore } from "../stores/schema";

  interface ForeignKeyInfo {
    constraint_name: string;
    table_schema: string;
    table_name: string;
    column_name: string;
    foreign_table_schema: string;
    foreign_table_name: string;
    foreign_column_name: string;
  }

  let foreignKeys = $state<ForeignKeyInfo[]>([]);
  let loading = $state(false);
  let error = $state<string | null>(null);

  async function loadForeignKeys() {
    if (!connectionStore.activeConnectionId) return;
    loading = true;
    error = null;
    try {
      foreignKeys = await invoke<ForeignKeyInfo[]>("get_foreign_keys", {
        connectionId: connectionStore.activeConnectionId,
      });
    } catch (err) {
      error = String(err);
    } finally {
      loading = false;
    }
  }

  // Auto-load when connection changes
  $effect(() => {
    if (connectionStore.activeConnectionId) {
      loadForeignKeys();
    } else {
      foreignKeys = [];
    }
  });

  interface TableBox {
    schema: string;
    name: string;
    x: number;
    y: number;
    width: number;
    height: number;
    columns: { name: string; data_type: string; is_pk: boolean }[];
  }

  let tables = $derived(() => {
    const result: TableBox[] = [];
    let x = 40;
    let y = 40;
    let maxHeightInRow = 0;

    for (const schema of schemaStore.schemas) {
      for (const table of schema.tables) {
        const cols = table.columns.map((c) => ({
          name: c.name,
          data_type: c.data_type,
          is_pk: c.is_primary_key,
        }));
        const height = 40 + cols.length * 22 + 16;
        const width = 240;

        result.push({
          schema: schema.name,
          name: table.name,
          x,
          y,
          width,
          height,
          columns: cols,
        });

        x += width + 60;
        if (height > maxHeightInRow) maxHeightInRow = height;

        if (x > 900) {
          x = 40;
          y += maxHeightInRow + 60;
          maxHeightInRow = 0;
        }
      }
    }
    return result;
  });

  let svgWidth = $derived(() => {
    const t = tables();
    if (t.length === 0) return 800;
    return Math.max(800, Math.max(...t.map((t) => t.x + t.width)) + 40);
  });

  let svgHeight = $derived(() => {
    const t = tables();
    if (t.length === 0) return 600;
    return Math.max(600, Math.max(...t.map((t) => t.y + t.height)) + 40);
  });

  function findTable(schema: string, name: string): TableBox | undefined {
    return tables().find((t) => t.schema === schema && t.name === name);
  }

  function getConnectionPoints(from: TableBox, to: TableBox): { x1: number; y1: number; x2: number; y2: number } {
    const cx1 = from.x + from.width / 2;
    const cy1 = from.y + from.height / 2;
    const cx2 = to.x + to.width / 2;
    const cy2 = to.y + to.height / 2;

    // Simple: connect from right side of from to left side of to
    let x1 = from.x + from.width;
    let y1 = cy1;
    let x2 = to.x;
    let y2 = cy2;

    if (cx2 < cx1) {
      x1 = from.x;
      x2 = to.x + to.width;
    }

    if (cy2 > cy1 + from.height) {
      y1 = from.y + from.height;
      y2 = to.y;
    } else if (cy2 < cy1 - from.height) {
      y1 = from.y;
      y2 = to.y + to.height;
    }

    return { x1, y1, x2, y2 };
  }

  function generatePath(x1: number, y1: number, x2: number, y2: number): string {
    const midX = (x1 + x2) / 2;
    return `M ${x1} ${y1} C ${midX} ${y1}, ${midX} ${y2}, ${x2} ${y2}`;
  }

  let connections = $derived(() => {
    const lines: { path: string; from: string; to: string }[] = [];
    for (const fk of foreignKeys) {
      const fromTable = findTable(fk.table_schema, fk.table_name);
      const toTable = findTable(fk.foreign_table_schema, fk.foreign_table_name);
      if (fromTable && toTable) {
        const pts = getConnectionPoints(fromTable, toTable);
        lines.push({
          path: generatePath(pts.x1, pts.y1, pts.x2, pts.y2),
          from: `${fk.table_schema}.${fk.table_name}.${fk.column_name}`,
          to: `${fk.foreign_table_schema}.${fk.foreign_table_name}.${fk.foreign_column_name}`,
        });
      }
    }
    return lines;
  });

  function exportSVG() {
    const svg = document.querySelector("#erd-svg");
    if (!svg) return;
    const blob = new Blob([svg.outerHTML], { type: "image/svg+xml" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `erd-${connectionStore.activeConnection?.name ?? "diagram"}.svg`;
    a.click();
    URL.revokeObjectURL(url);
  }
</script>

<div class="flex flex-col h-full bg-[#0c0c0c]">
  <div class="flex items-center justify-between px-3 py-1.5 bg-[#141414] border-b border-[#2a2a2a] shrink-0">
    <span class="text-[11px] text-[#6b6b6b] uppercase tracking-wider">ER Diagram</span>
    <div class="flex items-center gap-2">
      {#if foreignKeys.length > 0}
        <span class="text-[11px] text-[#a0a0a0] font-mono">{foreignKeys.length} relationships</span>
      {/if}
      <button
        onclick={exportSVG}
        disabled={tables().length === 0}
        class="text-[10px] font-medium px-2 py-0.5 bg-[#1a1a1a] border border-[#2a2a2a] rounded text-[#a0a0a0] hover:text-[#e8e8e8] hover:border-[#333333] disabled:opacity-50 disabled:cursor-not-allowed"
      >
        Export SVG
      </button>
    </div>
  </div>

  <div class="flex-1 overflow-auto relative">
    {#if loading}
      <div class="absolute inset-0 flex items-center justify-center">
        <div class="flex items-center gap-2 text-[#a0a0a0]">
          <div class="w-4 h-4 border-2 border-[#2a2a2a] border-t-[#00d4ff] rounded-full animate-spin"></div>
          <span class="text-[12px]">Loading relationships...</span>
        </div>
      </div>
    {:else if error}
      <div class="absolute inset-0 flex items-center justify-center">
        <div class="text-[#ef4444] text-[12px] font-mono max-w-lg">{error}</div>
      </div>
    {:else if tables().length === 0}
      <div class="absolute inset-0 flex items-center justify-center">
        <p class="text-[12px] text-[#6b6b6b]">Connect to a database to view ER diagram</p>
      </div>
    {:else}
      <svg id="erd-svg" width={svgWidth()} height={svgHeight()} class="block">
        <!-- Connections -->
        {#each connections() as conn}
          <path d={conn.path} fill="none" stroke="#2a4a5a" stroke-width="1.5" />
          <!-- Arrow head -->
          {@const pts = conn.path.match(/(\d+(?:\.\d+)?)\s+(\d+(?:\.\d+)?)$/)}
          {#if pts}
            <circle cx={Number(pts[1])} cy={Number(pts[2])} r="3" fill="#00d4ff" />
          {/if}
        {/each}

        <!-- Tables -->
        {#each tables() as t}
          <g>
            <!-- Table box -->
            <rect
              x={t.x}
              y={t.y}
              width={t.width}
              height={t.height}
              rx="4"
              fill="#141414"
              stroke="#2a2a2a"
              stroke-width="1"
            />
            <!-- Header -->
            <rect
              x={t.x}
              y={t.y}
              width={t.width}
              height="36"
              rx="4"
              fill="#1a1a1a"
            />
            <text
              x={t.x + t.width / 2}
              y={t.y + 22}
              text-anchor="middle"
              fill="#e8e8e8"
              font-size="12"
              font-family="ui-monospace, monospace"
              font-weight="600"
            >
              {t.schema}.{t.name}
            </text>
            <!-- Divider -->
            <line x1={t.x} y1={t.y + 36} x2={t.x + t.width} y2={t.y + 36} stroke="#2a2a2a" stroke-width="1" />
            <!-- Columns -->
            {#each t.columns as col, i}
              <text
                x={t.x + 12}
                y={t.y + 58 + i * 22}
                fill={col.is_pk ? "#00d4ff" : "#a0a0a0"}
                font-size="10"
                font-family="ui-monospace, monospace"
              >
                {col.is_pk ? "🔑 " : ""}{col.name} : {col.data_type}
              </text>
            {/each}
          </g>
        {/each}
      </svg>
    {/if}
  </div>
</div>
