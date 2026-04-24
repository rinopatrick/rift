<script lang="ts">
  import { queryStore } from "../stores/query";
</script>

<div class="flex items-center bg-[#0c0c0c] border-b border-[#2a2a2a] h-8 overflow-x-auto">
  {#each queryStore.tabs as tab}
    <div
      class="group flex items-center gap-1.5 px-3 h-full text-[12px] font-medium whitespace-nowrap border-t-2 transition-colors cursor-pointer {queryStore.activeTabId === tab.id ? 'bg-[#141414] border-t-[#00d4ff] text-[#e8e8e8]' : 'border-t-transparent text-[#6b6b6b] hover:text-[#a0a0a0]'}"
    >
      <button onclick={() => queryStore.activeTabId = tab.id} class="flex items-center gap-1.5">
        <span>{tab.name}</span>
        {#if tab.status === "running"}
          <span class="w-2 h-2 rounded-full bg-[#00d4ff] animate-pulse"></span>
        {/if}
      </button>
      <button
        onclick={(e) => { e.stopPropagation(); queryStore.removeTab(tab.id); }}
        class="opacity-0 group-hover:opacity-100 hover:text-[#ef4444] ml-1"
      >
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
      </button>
    </div>
  {/each}
  <button onclick={() => queryStore.addTab()} class="px-2 h-full text-[#6b6b6b] hover:text-[#a0a0a0]">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 5v14M5 12h14"/></svg>
  </button>
</div>
