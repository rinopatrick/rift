<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { connectionStore } from "../stores/connection";

  interface Props {
    onClose: () => void;
    onSuccess: () => void;
  }

  let { onClose, onSuccess }: Props = $props();

  let csvContent = $state("");
  let fileName = $state("");
  let tableName = $state("");
  let previewRows: string[][] = $state([]);
  let previewHeaders: string[] = $state([]);
  let detectedTypes: string[] = $state([]);
  let isDragging = $state(false);
  let isImporting = $state(false);
  let error = $state("");

  function parseCsv(content: string) {
    const lines = content.split("\n").filter((l) => l.trim());
    if (lines.length === 0) return;

    // Simple CSV parser (handles quotes roughly)
    const parseLine = (line: string): string[] => {
      const result: string[] = [];
      let current = "";
      let inQuotes = false;
      for (const char of line) {
        if (char === '"') {
          inQuotes = !inQuotes;
        } else if (char === "," && !inQuotes) {
          result.push(current.trim());
          current = "";
        } else {
          current += char;
        }
      }
      result.push(current.trim());
      return result;
    };

    previewHeaders = parseLine(lines[0]);
    previewRows = lines.slice(1, 11).map(parseLine);

    // Detect types from preview
    detectedTypes = previewHeaders.map((_, colIdx) => {
      let detected = "TEXT";
      for (const row of previewRows) {
        const val = row[colIdx]?.trim() ?? "";
        if (val === "") continue;
        if (!isNaN(Number(val)) && val !== "") {
          if (val.includes(".")) detected = "REAL";
          else if (detected !== "REAL") detected = "INTEGER";
        }
        const lower = val.toLowerCase();
        if (["true", "false", "1", "0", "yes", "no", "t", "f"].includes(lower)) {
          if (detected === "TEXT") detected = "BOOLEAN";
        }
      }
      return detected;
    });

    // Default table name from file
    if (!tableName) {
      tableName = fileName
        .replace(/\.csv$/i, "")
        .toLowerCase()
        .replace(/[^a-z0-9]/g, "_")
        .replace(/_+/g, "_")
        .replace(/^_|_$/g, "");
    }
  }

  function handleFileSelect(file: File) {
    fileName = file.name;
    const reader = new FileReader();
    reader.onload = (e) => {
      csvContent = String(e.target?.result ?? "");
      parseCsv(csvContent);
    };
    reader.readAsText(file);
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;
    const file = e.dataTransfer?.files[0];
    if (file && file.name.endsWith(".csv")) {
      handleFileSelect(file);
    }
  }

  async function handleImport() {
    if (!tableName || !csvContent || !connectionStore.activeConnectionId) return;
    isImporting = true;
    error = "";
    try {
      await invoke("import_csv", {
        connectionId: connectionStore.activeConnectionId,
        tableName,
        csvContent,
      });
      onSuccess();
      onClose();
    } catch (err) {
      error = String(err);
    } finally {
      isImporting = false;
    }
  }

  function handleFileInput(e: Event) {
    const input = e.target as HTMLInputElement;
    const file = input.files?.[0];
    if (file) handleFileSelect(file);
  }
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm" onclick={(e) => e.target === e.currentTarget && onClose()}>
  <div class="bg-[#141414] border border-[#2a2a2a] rounded-xl w-[640px] max-w-[90vw] max-h-[85vh] flex flex-col shadow-2xl">
    <!-- Header -->
    <div class="flex items-center justify-between px-5 py-3 border-b border-[#2a2a2a]">
      <h2 class="text-sm font-semibold text-[#e8e8e8]">Import CSV</h2>
      <button onclick={onClose} class="text-[#6b6b6b] hover:text-[#e8e8e8]">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
      </button>
    </div>

    <!-- Body -->
    <div class="flex-1 overflow-auto p-5 space-y-4">
      {#if !csvContent}
        <!-- Drop zone -->
        <div
          class="border-2 border-dashed rounded-lg p-10 text-center transition-colors {isDragging ? 'border-[#00d4ff] bg-[rgba(0,212,255,0.05)]' : 'border-[#2a2a2a] bg-[#0c0c0c]' }"
          ondragover={(e) => { e.preventDefault(); isDragging = true; }}
          ondragleave={() => isDragging = false}
          ondrop={handleDrop}
        >
          <svg class="mx-auto mb-3 text-[#4a4a4a]" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" y1="3" x2="12" y2="15"/></svg>
          <p class="text-sm text-[#a0a0a0] mb-1">Drop a CSV file here</p>
          <p class="text-xs text-[#4a4a4a] mb-3">or</p>
          <label class="inline-flex items-center gap-1.5 px-3 py-1.5 bg-[#1a1a1a] border border-[#2a2a2a] rounded text-[11px] text-[#a0a0a0] hover:text-[#e8e8e8] hover:border-[#333333] cursor-pointer">
            <input type="file" accept=".csv" class="hidden" onchange={handleFileInput} />
            Browse files
          </label>
        </div>
      {:else}
        <!-- Table name -->
        <div>
          <label class="block text-[11px] text-[#6b6b6b] uppercase tracking-wider mb-1.5">Table Name</label>
          <input
            bind:value={tableName}
            class="w-full bg-[#0c0c0c] border border-[#2a2a2a] rounded px-3 py-2 text-[12px] text-[#e8e8e8] outline-none focus:border-[#00d4ff]"
            placeholder="my_table"
          />
        </div>

        <!-- Preview -->
        <div>
          <div class="flex items-center justify-between mb-1.5">
            <label class="text-[11px] text-[#6b6b6b] uppercase tracking-wider">Preview ({previewRows.length} rows)</label>
            <button onclick={() => { csvContent = ""; fileName = ""; tableName = ""; previewRows = []; previewHeaders = []; }} class="text-[10px] text-[#4a4a4a] hover:text-[#ef4444]">Clear</button>
          </div>
          <div class="border border-[#2a2a2a] rounded overflow-auto max-h-56">
            <table class="w-full text-[11px]">
              <thead class="bg-[#1a1a1a] sticky top-0">
                <tr>
                  {#each previewHeaders as header, i}
                    <th class="text-left px-2 py-1.5 text-[#a0a0a0] font-medium whitespace-nowrap border-r border-[#2a2a2a] last:border-r-0">
                      {header}
                      <span class="text-[9px] text-[#00d4ff] ml-1 font-normal">{detectedTypes[i]?.toLowerCase()}</span>
                    </th>
                  {/each}
                </tr>
              </thead>
              <tbody>
                {#each previewRows as row}
                  <tr class="border-t border-[#1e1e1e]">
                    {#each row as cell}
                      <td class="px-2 py-1 text-[#e8e8e8] whitespace-nowrap border-r border-[#1e1e1e] last:border-r-0 max-w-[200px] truncate">{cell}</td>
                    {/each}
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>
      {/if}

      {#if error}
        <div class="text-[11px] text-[#ef4444] bg-[rgba(239,68,68,0.08)] border border-[rgba(239,68,68,0.2)] rounded px-3 py-2">{error}</div>
      {/if}
    </div>

    <!-- Footer -->
    <div class="flex items-center justify-end gap-2 px-5 py-3 border-t border-[#2a2a2a]">
      <button onclick={onClose} class="px-3 py-1.5 text-[11px] text-[#a0a0a0] hover:text-[#e8e8e8]">Cancel</button>
      <button
        onclick={handleImport}
        disabled={!csvContent || !tableName || isImporting}
        class="flex items-center gap-1.5 px-3 py-1.5 text-[11px] font-medium bg-[#00d4ff] text-black rounded hover:brightness-110 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {#if isImporting}
          <div class="w-3 h-3 border-2 border-black/30 border-t-black rounded-full animate-spin"></div>
          Importing...
        {:else}
          Import
        {/if}
      </button>
    </div>
  </div>
</div>
