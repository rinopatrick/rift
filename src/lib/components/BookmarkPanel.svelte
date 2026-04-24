<script lang="ts">
  import { bookmarkStore } from "../stores/bookmark";
  import { connectionStore } from "../stores/connection";
  import { queryStore } from "../stores/query";

  let visible = $state(false);
  let newName = $state("");

  $effect(() => {
    if (connectionStore.activeConnectionId) {
      bookmarkStore.load(connectionStore.activeConnectionId);
    }
  });

  async function saveCurrent() {
    const activeTab = queryStore.tabs.find((t) => t.id === queryStore.activeTabId);
    if (!connectionStore.activeConnectionId || !activeTab?.sql.trim()) return;
    const name = newName.trim() || `Bookmark ${bookmarkStore.bookmarks.length + 1}`;
    await bookmarkStore.save(connectionStore.activeConnectionId, name, activeTab.sql);
    newName = "";
  }

  function loadBookmark(query: string) {
    const activeTab = queryStore.tabs.find((t) => t.id === queryStore.activeTabId);
    if (activeTab) {
      queryStore.updateSql(activeTab.id, query);
    }
  }
</script>

<div class="border-t border-[#2a2a2a]">
  <button
    onclick={() => visible = !visible}
    class="w-full flex items-center justify-between px-3 py-2 text-[11px] font-medium text-[#6b6b6b] uppercase tracking-wider hover:text-[#a0a0a0]"
  >
    <span>Bookmarks</span>
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="transition-transform {visible ? 'rotate-90' : ''}">
      <path d="m9 18 6-6-6-6"/>
    </svg>
  </button>

  {#if visible}
    <div class="px-2 pb-2">
      <div class="flex gap-1 mb-2">
        <input
          bind:value={newName}
          onkeydown={(e) => e.key === "Enter" && saveCurrent()}
          placeholder="Name..."
          class="flex-1 bg-[#0c0c0c] text-[#e8e8e8] text-[11px] px-2 py-1 rounded border border-[#2a2a2a] focus:border-[#00d4ff] focus:outline-none"
        />
        <button
          onclick={saveCurrent}
          class="text-[10px] px-2 py-1 bg-[#1a1a1a] border border-[#2a2a2a] rounded text-[#a0a0a0] hover:text-[#e8e8e8]"
        >Save</button>
      </div>
      <div class="max-h-32 overflow-auto">
        {#each bookmarkStore.bookmarks as bm}
          <div class="group flex items-center justify-between px-2 py-1 rounded hover:bg-[#1a1a1a]">
            <button
              onclick={() => loadBookmark(bm.query)}
              class="text-left text-[11px] text-[#a0a0a0] hover:text-[#e8e8e8] truncate flex-1"
              title={bm.query}
            >
              {bm.name}
            </button>
            <button
              onclick={() => connectionStore.activeConnectionId && bookmarkStore.remove(bm.id, connectionStore.activeConnectionId)}
              class="opacity-0 group-hover:opacity-100 text-[#6b6b6b] hover:text-[#ef4444] ml-1"
            >
              <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
            </button>
          </div>
        {:else}
          <p class="text-[10px] text-[#4a4a4a] px-2 py-1">No bookmarks yet</p>
        {/each}
      </div>
    </div>
  {/if}
</div>
