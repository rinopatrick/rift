<script lang="ts">
  let sidebarWidth = 240;
</script>

<div class="flex h-screen w-screen bg-rift-base overflow-hidden">
  <aside class="shrink-0 bg-rift-surface border-r border-[#2a2a2a] flex flex-col" style="width: {sidebarWidth}px">
    <div class="px-3 py-2 border-b border-[#2a2a2a]">
      <span class="text-sm font-semibold tracking-tight">Rift</span>
    </div>
    <div class="flex-1 p-2">
      <p class="text-xs text-[#6b6b6b]">Connections will appear here</p>
    </div>
  </aside>

  <main class="flex-1 flex flex-col min-w-0">
    <div class="flex-1 flex items-center justify-center">
      <p class="text-sm text-[#6b6b6b]">Select a connection to start</p>
    </div>
    <div class="h-6 bg-rift-base border-t border-[#1e1e1e] flex items-center px-3 text-[11px] text-[#6b6b6b] font-mono">
      <span>Ready</span>
    </div>
  </main>
</div>
