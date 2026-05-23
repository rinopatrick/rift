<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { ConnectionInfo } from '../types';

  let { connections }: { connections: ConnectionInfo[] } = $props();
  let sourceId = $state('');
  let targetId = $state('');
  let loading = $state(false);
  let error = $state('');
  let result: any = $state(null);

  async function runDiff() {
    if (!sourceId || !targetId) return;
    loading = true;
    error = '';
    try {
      result = await invoke('diff_schemas', {
        sourceConnectionId: sourceId,
        targetConnectionId: targetId,
      });
    } catch (e: any) {
      error = e.toString();
    }
    loading = false;
  }

  const pgConns = $derived(connections.filter(c => c.driver === 'postgres'));
</script>

<div class="p-6 text-gray-300">
  <h2 class="text-xl font-semibold mb-4">Schema Diff</h2>

  <div class="flex gap-4 mb-6 items-end">
    <div class="flex-1">
      <label for="source-conn" class="block text-xs text-gray-500 mb-1">Source Connection</label>
      <select id="source-conn" bind:value={sourceId} class="w-full bg-gray-800 border border-gray-700 rounded px-3 py-2 text-sm">
        <option value="">Select source...</option>
        {#each pgConns as conn}
          <option value={conn.id}>{conn.name}</option>
        {/each}
      </select>
    </div>
    <div class="text-gray-500 pb-2">→</div>
    <div class="flex-1">
      <label for="target-conn" class="block text-xs text-gray-500 mb-1">Target Connection</label>
      <select id="target-conn" bind:value={targetId} class="w-full bg-gray-800 border border-gray-700 rounded px-3 py-2 text-sm">
        <option value="">Select target...</option>
        {#each pgConns as conn}
          <option value={conn.id}>{conn.name}</option>
        {/each}
      </select>
    </div>
    <button
      onclick={runDiff}
      disabled={!sourceId || !targetId || loading}
      class="bg-blue-600 hover:bg-blue-500 disabled:opacity-50 disabled:cursor-not-allowed text-white px-4 py-2 rounded text-sm font-medium"
    >
      {loading ? 'Comparing...' : 'Compare'}
    </button>
  </div>

  {#if error}
    <div class="bg-red-900/30 border border-red-800 text-red-300 px-4 py-3 rounded mb-4 text-sm">{error}</div>
  {/if}

  {#if result}
    <div class="space-y-6">
      {#if result.added_tables.length > 0}
        <section>
          <h3 class="text-sm font-medium text-green-400 mb-2 flex items-center gap-2">
            <span class="bg-green-900/40 text-green-400 px-2 py-0.5 rounded text-xs">+{result.added_tables.length}</span>
            Added Tables
          </h3>
          <div class="space-y-2">
            {#each result.added_tables as table}
              <div class="bg-gray-800/50 border border-gray-700 rounded p-3">
                <div class="font-medium text-sm text-green-300">{table.name}</div>
                <div class="mt-2 space-y-1">
                  {#each table.columns as col}
                    <div class="text-xs text-gray-400 flex gap-2">
                      <span class="text-gray-300">{col.name}</span>
                      <span class="text-gray-500">{col.data_type}</span>
                      {#if !col.is_nullable}<span class="text-red-400">NOT NULL</span>{/if}
                    </div>
                  {/each}
                </div>
              </div>
            {/each}
          </div>
        </section>
      {/if}

      {#if result.removed_tables.length > 0}
        <section>
          <h3 class="text-sm font-medium text-red-400 mb-2 flex items-center gap-2">
            <span class="bg-red-900/40 text-red-400 px-2 py-0.5 rounded text-xs">-{result.removed_tables.length}</span>
            Removed Tables
          </h3>
          <div class="space-y-2">
            {#each result.removed_tables as table}
              <div class="bg-gray-800/50 border border-gray-700 rounded p-3">
                <div class="font-medium text-sm text-red-300">{table.name}</div>
                <div class="mt-2 space-y-1">
                  {#each table.columns as col}
                    <div class="text-xs text-gray-500 flex gap-2">
                      <span>{col.name}</span>
                      <span>{col.data_type}</span>
                    </div>
                  {/each}
                </div>
              </div>
            {/each}
          </div>
        </section>
      {/if}

      {#if result.modified_tables.length > 0}
        <section>
          <h3 class="text-sm font-medium text-yellow-400 mb-2 flex items-center gap-2">
            <span class="bg-yellow-900/40 text-yellow-400 px-2 py-0.5 rounded text-xs">~{result.modified_tables.length}</span>
            Modified Tables
          </h3>
          <div class="space-y-2">
            {#each result.modified_tables as table}
              <div class="bg-gray-800/50 border border-gray-700 rounded p-3">
                <div class="font-medium text-sm text-yellow-300">{table.name}</div>

                {#if table.added_columns.length > 0}
                  <div class="mt-2">
                    <div class="text-xs text-green-500 mb-1">Added columns:</div>
                    {#each table.added_columns as col}
                      <div class="text-xs text-gray-400 flex gap-2 ml-2">
                        <span class="text-green-400">+ {col.name}</span>
                        <span class="text-gray-500">{col.data_type}</span>
                      </div>
                    {/each}
                  </div>
                {/if}

                {#if table.removed_columns.length > 0}
                  <div class="mt-2">
                    <div class="text-xs text-red-500 mb-1">Removed columns:</div>
                    {#each table.removed_columns as col}
                      <div class="text-xs text-gray-500 flex gap-2 ml-2">
                        <span class="text-red-400">- {col.name}</span>
                        <span>{col.data_type}</span>
                      </div>
                    {/each}
                  </div>
                {/if}

                {#if table.modified_columns.length > 0}
                  <div class="mt-2">
                    <div class="text-xs text-yellow-500 mb-1">Changed columns:</div>
                    {#each table.modified_columns as col}
                      <div class="text-xs text-gray-400 flex gap-2 ml-2 items-center">
                        <span class="text-yellow-400">~ {col.name}</span>
                        <span class="text-gray-500">{col.old_type} → {col.new_type}</span>
                        {#if col.old_nullable !== col.new_nullable}
                          <span class="text-gray-500">({col.old_nullable ? 'NULL' : 'NOT NULL'} → {col.new_nullable ? 'NULL' : 'NOT NULL'})</span>
                        {/if}
                      </div>
                    {/each}
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        </section>
      {/if}

      {#if result.added_tables.length === 0 && result.removed_tables.length === 0 && result.modified_tables.length === 0}
        <div class="text-center text-gray-500 py-8 text-sm">No differences found — schemas are identical.</div>
      {/if}
    </div>
  {/if}
</div>
