<script lang="ts">
  import { uiStore } from "../stores/ui";
  import { connectionStore } from "../stores/connection";
  import type { ConnectionConfig } from "../types";

  let name = $state("");
  let host = $state("localhost");
  let port = $state(5432);
  let database = $state("postgres");
  let username = $state("postgres");
  let password = $state("");
  let testing = $state(false);
  let testResult = $state<boolean | null>(null);

  async function handleTest() {
    testing = true;
    testResult = null;
    try {
      const config = { id: "", name, host, port, database, username, password, ssl_mode: "prefer", created_at: "" } as ConnectionConfig;
      testResult = await connectionStore.testConnection(config);
    } finally {
      testing = false;
    }
  }

  async function handleSave() {
    const config = { id: "", name, host, port, database, username, password, ssl_mode: "prefer", created_at: "" } as ConnectionConfig;
    await connectionStore.saveConnection(config);
    uiStore.showConnectionModal = false;
    reset();
  }

  function reset() {
    name = "";
    host = "localhost";
    port = 5432;
    database = "postgres";
    username = "postgres";
    password = "";
    testResult = null;
  }
</script>

{#if uiStore.showConnectionModal}
<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm">
  <div class="w-full max-w-md bg-[#141414] border border-[#333333] rounded-lg shadow-xl">
    <div class="flex items-center justify-between px-4 py-3 border-b border-[#2a2a2a]">
      <h3 class="text-sm font-semibold text-[#e8e8e8]">New Connection</h3>
      <button onclick={() => uiStore.showConnectionModal = false} class="text-[#6b6b6b] hover:text-[#e8e8e8]">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
      </button>
    </div>
    <div class="p-4 space-y-3">
      <div>
        <label for="conn-name" class="block text-[11px] font-medium text-[#a0a0a0] mb-1">Name</label>
        <input id="conn-name" type="text" bind:value={name} class="w-full bg-[#1a1a1a] border border-[#2a2a2a] rounded px-3 py-1.5 text-[13px] text-[#e8e8e8] focus:border-[#00d4ff] focus:outline-none" placeholder="My Database" />
      </div>
      <div class="grid grid-cols-3 gap-2">
        <div class="col-span-2">
          <label for="conn-host" class="block text-[11px] font-medium text-[#a0a0a0] mb-1">Host</label>
          <input id="conn-host" type="text" bind:value={host} class="w-full bg-[#1a1a1a] border border-[#2a2a2a] rounded px-3 py-1.5 text-[13px] text-[#e8e8e8] focus:border-[#00d4ff] focus:outline-none" />
        </div>
        <div>
          <label for="conn-port" class="block text-[11px] font-medium text-[#a0a0a0] mb-1">Port</label>
          <input id="conn-port" type="number" bind:value={port} class="w-full bg-[#1a1a1a] border border-[#2a2a2a] rounded px-3 py-1.5 text-[13px] text-[#e8e8e8] focus:border-[#00d4ff] focus:outline-none" />
        </div>
      </div>
      <div>
        <label for="conn-db" class="block text-[11px] font-medium text-[#a0a0a0] mb-1">Database</label>
        <input id="conn-db" type="text" bind:value={database} class="w-full bg-[#1a1a1a] border border-[#2a2a2a] rounded px-3 py-1.5 text-[13px] text-[#e8e8e8] focus:border-[#00d4ff] focus:outline-none" />
      </div>
      <div class="grid grid-cols-2 gap-2">
        <div>
          <label for="conn-user" class="block text-[11px] font-medium text-[#a0a0a0] mb-1">Username</label>
          <input id="conn-user" type="text" bind:value={username} class="w-full bg-[#1a1a1a] border border-[#2a2a2a] rounded px-3 py-1.5 text-[13px] text-[#e8e8e8] focus:border-[#00d4ff] focus:outline-none" />
        </div>
        <div>
          <label for="conn-pass" class="block text-[11px] font-medium text-[#a0a0a0] mb-1">Password</label>
          <input id="conn-pass" type="password" bind:value={password} class="w-full bg-[#1a1a1a] border border-[#2a2a2a] rounded px-3 py-1.5 text-[13px] text-[#e8e8e8] focus:border-[#00d4ff] focus:outline-none" />
        </div>
      </div>
      {#if testResult !== null}
        <div class="text-[12px] {testResult ? 'text-green-400' : 'text-red-400'}">
          {testResult ? 'Connection successful!' : 'Connection failed'}
        </div>
      {/if}
    </div>
    <div class="flex justify-end gap-2 px-4 py-3 border-t border-[#2a2a2a]">
      <button onclick={handleTest} disabled={testing} class="px-3 py-1.5 text-[12px] font-medium bg-[#1a1a1a] border border-[#2a2a2a] rounded text-[#e8e8e8] hover:border-[#333333]">
        {testing ? 'Testing...' : 'Test'}
      </button>
      <button onclick={handleSave} class="px-3 py-1.5 text-[12px] font-medium bg-[#00d4ff] text-black rounded hover:brightness-110">
        Save
      </button>
    </div>
  </div>
</div>
{/if}
