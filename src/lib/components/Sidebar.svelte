<script lang="ts">
  import { uiStore } from "../stores/ui";
  import ConnectionList from "./ConnectionList.svelte";
  import SchemaTree from "./SchemaTree.svelte";
  import BookmarkPanel from "./BookmarkPanel.svelte";
  import CsvImportModal from "./CsvImportModal.svelte";

  let width = $state(240);
  let showImportModal = $state(false);
</script>

<aside class="shrink-0 bg-[#141414] border-r border-[#2a2a2a] flex flex-col h-full" style="width: {uiStore.sidebarCollapsed ? 48 : width}px">
  <div class="flex items-center px-3 py-2 border-b border-[#2a2a2a] h-9">
    {#if !uiStore.sidebarCollapsed}
      <span class="text-[14px] font-semibold text-[#e8e8e8] tracking-tight">Rift</span>
    {/if}
    <button
      onclick={() => uiStore.sidebarCollapsed = !uiStore.sidebarCollapsed}
      class="ml-auto text-[#6b6b6b] hover:text-[#a0a0a0]"
    >
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        {#if uiStore.sidebarCollapsed}
          <path d="m9 18 6-6-6-6"/>
        {:else}
          <path d="m15 18-6-6 6-6"/>
        {/if}
      </svg>
    </button>
  </div>
  {#if !uiStore.sidebarCollapsed}
    <div class="flex-1 flex flex-col min-h-0 overflow-hidden">
      <ConnectionList />
      <div class="border-t border-[#2a2a2a] flex-1 min-h-0 flex flex-col">
        <SchemaTree />
        <BookmarkPanel />
      </div>
      <!-- Quick Actions -->
      <div class="border-t border-[#2a2a2a] p-2 space-y-1">
        <button
          onclick={() => showImportModal = true}
          class="w-full flex items-center gap-2 px-3 py-2 rounded-md text-[11px] text-[#a0a0a0] hover:text-[#e8e8e8] hover:bg-[#1f1f1f] transition-colors"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" y1="3" x2="12" y2="15"/></svg>
          Import CSV
        </button>
      </div>
    </div>
  {/if}
</aside>

{#if showImportModal}
  <CsvImportModal onClose={() => showImportModal = false} onSuccess={() => { /* schema refresh handled by parent */ }} />
{/if}
