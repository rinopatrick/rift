<script lang="ts">
  import Sidebar from "./lib/components/Sidebar.svelte";
  import ConnectionModal from "./lib/components/ConnectionModal.svelte";
  import TabBar from "./lib/components/TabBar.svelte";
  import QueryEditor from "./lib/components/QueryEditor.svelte";
  import ResultGrid from "./lib/components/ResultGrid.svelte";
  import StatusBar from "./lib/components/StatusBar.svelte";
</script>

<div class="flex h-screen w-screen bg-[#0c0c0c] overflow-hidden">
  <Sidebar />

  <main class="flex-1 flex flex-col min-w-0">
    <TabBar />
    <div class="flex-1 flex flex-col min-h-0">
      <div class="h-[45%] min-h-[120px]">
        <QueryEditor />
      </div>
      <div class="h-[55%] min-h-[120px] border-t border-[#2a2a2a]">
        <ResultGrid />
      </div>
    </div>
    <StatusBar />
  </main>
</div>

<ConnectionModal />
