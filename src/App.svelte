<script lang="ts">
  import Sidebar from "./lib/components/Sidebar.svelte";
  import ConnectionModal from "./lib/components/ConnectionModal.svelte";
  import TabBar from "./lib/components/TabBar.svelte";
  import QueryEditor from "./lib/components/QueryEditor.svelte";
  import ResultGrid from "./lib/components/ResultGrid.svelte";
  import StatusBar from "./lib/components/StatusBar.svelte";
  import ERDiagram from "./lib/components/ERDiagram.svelte";
  import { settingsStore } from "./lib/stores/settings";
  import { connectionStore } from "./lib/stores/connection";

  let mainView = $state<"query" | "erd">("query");

  $effect(() => {
    settingsStore.load();
  });
</script>

<div class="flex h-screen w-screen bg-[#0c0c0c] overflow-hidden">
  <Sidebar />

  <main class="flex-1 flex flex-col min-w-0">
    <TabBar />
    <div class="flex items-center justify-between px-3 py-1 bg-[#141414] border-b border-[#2a2a2a]">
      <div class="flex items-center gap-1">
        <button
          onclick={() => mainView = "query"}
          class="text-[10px] font-medium px-2 py-0.5 rounded {mainView === 'query' ? 'bg-[#00d4ff] text-black' : 'text-[#a0a0a0] hover:text-[#e8e8e8]'}"
        >
          Query
        </button>
        <button
          onclick={() => mainView = "erd"}
          disabled={!connectionStore.activeConnectionId}
          class="text-[10px] font-medium px-2 py-0.5 rounded {mainView === 'erd' ? 'bg-[#00d4ff] text-black' : 'text-[#a0a0a0] hover:text-[#e8e8e8]'} disabled:opacity-50 disabled:cursor-not-allowed"
        >
          ER Diagram
        </button>
      </div>
    </div>
    {#if mainView === "query"}
      <div class="flex-1 flex flex-col min-h-0">
        <div class="h-[45%] min-h-[120px]">
          <QueryEditor />
        </div>
        <div class="h-[55%] min-h-[120px] border-t border-[#2a2a2a]">
          <ResultGrid />
        </div>
      </div>
    {:else}
      <div class="flex-1 min-h-0">
        <ERDiagram />
      </div>
    {/if}
    <StatusBar />
  </main>
</div>

<ConnectionModal />
