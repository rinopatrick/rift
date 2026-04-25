<script lang="ts">
  import { uiStore } from "../stores/ui";
  import ConnectionList from "./ConnectionList.svelte";
  import SchemaTree from "./SchemaTree.svelte";
  import BookmarkPanel from "./BookmarkPanel.svelte";
  import CsvImportModal from "./CsvImportModal.svelte";
  import SettingsPanel from "./SettingsPanel.svelte";

  let width = $state(240);
  let showImportModal = $state(false);
  let showSettingsPanel = $state(false);
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
        <button
          onclick={() => showSettingsPanel = true}
          class="w-full flex items-center gap-2 px-3 py-2 rounded-md text-[11px] text-[#a0a0a0] hover:text-[#e8e8e8] hover:bg-[#1f1f1f] transition-colors"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.68 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>
          Settings
        </button>
      </div>
    </div>
  {/if}
</aside>

{#if showSettingsPanel}
  <SettingsPanel onClose={() => showSettingsPanel = false} />
{/if}
{#if showImportModal}
  <CsvImportModal onClose={() => showImportModal = false} onSuccess={() => { /* schema refresh handled by parent */ }} />
{/if}
