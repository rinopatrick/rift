<script lang="ts">
  import { connectionStore } from "../stores/connection";
  import { uiStore } from "../stores/ui";

  $effect(() => {
    connectionStore.loadConnections();
  });

  let groupedConnections = $derived(() => {
    const groups = new Map<string, typeof connectionStore.connections>();
    const ungrouped: typeof connectionStore.connections = [];
    
    for (const conn of connectionStore.connections) {
      if (conn.folder) {
        if (!groups.has(conn.folder)) {
          groups.set(conn.folder, []);
        }
        groups.get(conn.folder)!.push(conn);
      } else {
        ungrouped.push(conn);
      }
    }
    
    return { groups: Array.from(groups.entries()).sort((a, b) => a[0].localeCompare(b[0])), ungrouped };
  });

  let expandedFolders = $state<Set<string>>(new Set());

  function toggleFolder(name: string) {
    const next = new Set(expandedFolders);
    if (next.has(name)) {
      next.delete(name);
    } else {
      next.add(name);
    }
    expandedFolders = next;
  }
</script>

<div class="p-2">
  <div class="flex items-center justify-between mb-2">
    <span class="text-[11px] font-medium text-[#6b6b6b] uppercase tracking-wider">Connections</span>
    <button onclick={() => uiStore.showConnectionModal = true} class="text-[#00d4ff] hover:text-[#0ea5e9]">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 5v14M5 12h14"/></svg>
    </button>
  </div>
  <div class="space-y-1">
    <!-- Ungrouped connections -->
    {#each groupedConnections().ungrouped as conn}
      <button
        onclick={() => connectionStore.connect(conn.id)}
        class="w-full text-left px-3 py-2 rounded-md text-[12px] border transition-colors {connectionStore.activeConnectionId === conn.id ? 'bg-[#262626] border-l-[3px] border-l-[#00d4ff] border-[#262626]' : 'bg-[#141414] border-[#2a2a2a] hover:bg-[#1f1f1f] hover:border-[#333333]' }"
      >
        <div class="flex items-center justify-between">
          <div class="font-medium text-[#e8e8e8]">{conn.name}</div>
          <span class="text-[9px] px-1 py-0.5 rounded bg-[#1a1a1a] text-[#6b6b6b] uppercase">{conn.driver}</span>
        </div>
        <div class="text-[#6b6b6b]">{conn.host}:{conn.port}/{conn.database}</div>
      </button>
    {/each}

    <!-- Grouped connections -->
    {#each groupedConnections().groups as [folderName, connections]}
      <div class="mt-2">
        <button
          onclick={() => toggleFolder(folderName)}
          class="w-full flex items-center gap-1.5 px-2 py-1 text-[11px] font-medium text-[#a0a0a0] hover:text-[#e8e8e8]"
        >
          <span class="text-[10px]">{expandedFolders.has(folderName) ? "\u25bc" : "\u25b6"}</span>
          <span>{folderName}</span>
          <span class="text-[#4a4a4a]">({connections.length})</span>
        </button>
        {#if expandedFolders.has(folderName)}
          <div class="space-y-1 mt-1 ml-2 border-l border-[#2a2a2a] pl-2">
            {#each connections as conn}
              <button
                onclick={() => connectionStore.connect(conn.id)}
                class="w-full text-left px-3 py-2 rounded-md text-[12px] border transition-colors {connectionStore.activeConnectionId === conn.id ? 'bg-[#262626] border-l-[3px] border-l-[#00d4ff] border-[#262626]' : 'bg-[#141414] border-[#2a2a2a] hover:bg-[#1f1f1f] hover:border-[#333333]' }"
              >
                <div class="flex items-center justify-between">
                  <div class="font-medium text-[#e8e8e8]">{conn.name}</div>
                  <span class="text-[9px] px-1 py-0.5 rounded bg-[#1a1a1a] text-[#6b6b6b] uppercase">{conn.driver}</span>
                </div>
                <div class="text-[#6b6b6b]">{conn.host}:{conn.port}/{conn.database}</div>
              </button>
            {/each}
          </div>
        {/if}
      </div>
    {/each}

    {#if connectionStore.connections.length === 0}
      <p class="text-[11px] text-[#6b6b6b] px-2">No connections yet</p>
    {/if}
  </div>
</div>
