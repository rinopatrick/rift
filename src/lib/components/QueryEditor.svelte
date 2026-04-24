<script lang="ts">
  import { queryStore } from "../stores/query";
  import { connectionStore } from "../stores/connection";

  let activeTab = $derived(queryStore.tabs.find((t) => t.id === queryStore.activeTabId));

  function handleKeyDown(e: KeyboardEvent) {
    if ((e.metaKey || e.ctrlKey) && e.key === "Enter") {
      e.preventDefault();
      if (connectionStore.activeConnectionId && activeTab) {
        queryStore.executeQuery(connectionStore.activeConnectionId, activeTab.id);
      }
    }
  }
</script>

<div class="flex flex-col h-full">
  <div class="flex items-center justify-between px-3 py-1.5 bg-[#141414] border-b border-[#2a2a2a]">
    <span class="text-[11px] text-[#6b6b6b] uppercase tracking-wider">Query Editor</span>
    <button
      onclick={() => {
        if (connectionStore.activeConnectionId && activeTab) {
          queryStore.executeQuery(connectionStore.activeConnectionId, activeTab.id);
        }
      }}
      disabled={!connectionStore.activeConnectionId || activeTab?.status === "running"}
      class="flex items-center gap-1.5 px-3 py-1 text-[11px] font-medium bg-[#00d4ff] text-black rounded hover:brightness-110 disabled:opacity-50 disabled:cursor-not-allowed"
    >
      <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M5 12h14"/><path d="m12 5 7 7-7 7"/></svg>
      Run (Ctrl+Enter)
    </button>
  </div>
  <textarea
    value={activeTab?.sql ?? ""}
    oninput={(e) => {
      if (activeTab) queryStore.updateSql(activeTab.id, e.currentTarget.value);
    }}
    onkeydown={handleKeyDown}
    class="flex-1 w-full bg-[#0c0c0c] text-[#e8e8e8] font-mono text-[13px] p-3 resize-none focus:outline-none leading-relaxed"
    placeholder="-- Write your SQL query here
SELECT * FROM users LIMIT 100;"
    spellcheck="false"
  ></textarea>
</div>
