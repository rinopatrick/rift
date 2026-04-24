<script lang="ts">
  import { uiStore } from "../stores/ui";
  import ConnectionList from "./ConnectionList.svelte";
  import SchemaTree from "./SchemaTree.svelte";

  let width = $state(240);
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
      </div>
    </div>
  {/if}
</aside>
