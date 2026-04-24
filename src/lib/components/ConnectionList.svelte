<script lang="ts">
  import { connectionStore } from "../stores/connection";
  import { uiStore } from "../stores/ui";

  $effect(() => {
    connectionStore.loadConnections();
  });
</script>

<div class="p-2">
  <div class="flex items-center justify-between mb-2">
    <span class="text-[11px] font-medium text-[#6b6b6b] uppercase tracking-wider">Connections</span>
    <button onclick={() => uiStore.showConnectionModal = true} class="text-[#00d4ff] hover:text-[#0ea5e9]">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 5v14M5 12h14"/></svg>
    </button>
  </div>
  <div class="space-y-1">
    {#each connectionStore.connections as conn}
      <button
        onclick={() => connectionStore.connect(conn.id)}
        class="w-full text-left px-3 py-2 rounded-md text-[12px] border transition-colors {connectionStore.activeConnectionId === conn.id ? 'bg-[#262626] border-l-[3px] border-l-[#00d4ff] border-[#262626]' : 'bg-[#141414] border-[#2a2a2a] hover:bg-[#1f1f1f] hover:border-[#333333]' }"
      >
        <div class="font-medium text-[#e8e8e8]">{conn.name}</div>
        <div class="text-[#6b6b6b]">{conn.host}:{conn.port}/{conn.database}</div>
      </button>
    {:else}
      <p class="text-[11px] text-[#6b6b6b] px-2">No connections yet</p>
    {/each}
  </div>
</div>
