<script lang="ts">
  import { schemaStore } from "../stores/schema";
  import { queryStore } from "../stores/query";

  function useTable(schema: string, table: string) {
    const sql = `SELECT * FROM "${schema}"."${table}" LIMIT 100;`;
    const activeTab = queryStore.tabs.find((t) => t.id === queryStore.activeTabId);
    if (activeTab) {
      queryStore.updateSql(activeTab.id, sql);
    }
  }
</script>

<div class="flex-1 overflow-y-auto p-2">
  <div class="flex items-center justify-between mb-2">
    <span class="text-[11px] font-medium text-[#6b6b6b] uppercase tracking-wider">Schema</span>
    {#if schemaStore.loading}
      <div class="w-3 h-3 border-2 border-[#2a2a2a] border-t-[#00d4ff] rounded-full animate-spin"></div>
    {/if}
  </div>

  {#each schemaStore.schemas as schema}
    <div class="mb-1">
      <button
        onclick={() => schemaStore.toggleSchema(schema.name)}
        class="flex items-center gap-1.5 w-full px-2 py-1 text-[12px] text-[#a0a0a0] hover:text-[#e8e8e8] hover:bg-[#1f1f1f] rounded"
      >
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="transition-transform {schemaStore.expandedSchemas.has(schema.name) ? 'rotate-90' : ''}">
          <path d="m9 18 6-6-6-6"/>
        </svg>
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="text-[#6b6b6b]">
          <path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"/>
        </svg>
        <span class="font-medium">{schema.name}</span>
      </button>

      {#if schemaStore.expandedSchemas.has(schema.name)}
        <div class="ml-5 border-l border-[#1e1e1e]">
          {#each schema.tables as table}
            <div>
              <div
                class="group flex items-center gap-1.5 w-full px-2 py-1 text-[12px] text-[#a0a0a0] hover:text-[#e8e8e8] hover:bg-[#1f1f1f] rounded cursor-pointer"
              >
                <button onclick={() => schemaStore.toggleTable(`${schema.name}.${table.name}`)} class="flex items-center gap-1.5 flex-1 text-left">
                  <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="transition-transform {schemaStore.expandedTables.has(`${schema.name}.${table.name}`) ? 'rotate-90' : ''}">
                    <path d="m9 18 6-6-6-6"/>
                  </svg>
                  <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="text-[#6b6b6b]">
                    <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
                    <line x1="3" y1="9" x2="21" y2="9"/>
                    <line x1="9" y1="21" x2="9" y2="9"/>
                  </svg>
                  <span>{table.name}</span>
                </button>
                <button
                  onclick={(e) => { e.stopPropagation(); useTable(schema.name, table.name); }}
                  class="ml-auto text-[#6b6b6b] hover:text-[#00d4ff] opacity-0 group-hover:opacity-100"
                  title="SELECT * FROM this table"
                >
                  <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M5 12h14"/><path d="m12 5 7 7-7 7"/>
                  </svg>
                </button>
              </div>

              {#if schemaStore.expandedTables.has(`${schema.name}.${table.name}`)}
                <div class="ml-4 border-l border-[#1e1e1e]">
                  {#each table.columns as col}
                    <div class="flex items-center gap-1.5 px-2 py-0.5 text-[11px]">
                      {#if col.is_primary_key}
                        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#00d4ff" stroke-width="2">
                          <path d="m21 2-2 2m-7.61 7.61a5.5 5.5 0 1 1-7.778 7.778 5.5 5.5 0 0 1 7.777-7.777zm0 0L15.5 7.5m0 0 3 3L22 7l-3-3m-3.5 3.5L19 4"/>
                        </svg>
                      {:else}
                        <span class="w-[10px]"></span>
                      {/if}
                      <span class="text-[#a0a0a0]">{col.name}</span>
                      <span class="text-[#6b6b6b]">{col.data_type}</span>
                      {#if !col.is_nullable}
                        <span class="text-[#6b6b6b]">!</span>
                      {/if}
                    </div>
                  {/each}
                </div>
              {/if}
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {:else}
    {#if !schemaStore.loading}
      <p class="text-[11px] text-[#6b6b6b] px-2">Connect to a database to explore schema</p>
    {/if}
  {/each}
</div>
