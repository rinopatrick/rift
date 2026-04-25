<script lang="ts">
  import { uiStore } from "../stores/ui";
  import { connectionStore } from "../stores/connection";
  import type { ConnectionConfig } from "../types";

  let driver = $state<"postgres" | "mysql" | "sqlite">("postgres");
  let name = $state("");
  let host = $state("localhost");
  let port = $state(5432);
  let database = $state("postgres");
  let username = $state("postgres");
  let password = $state("");
  let filePath = $state("");
  let folder = $state("");
  let useSshTunnel = $state(false);
  let sshHost = $state("");
  let sshPort = $state(22);
  let sshUsername = $state("");
  let sshPassword = $state("");
  let sshPrivateKey = $state("");
  let sshPassphrase = $state("");
  let testing = $state(false);
  let testResult = $state<boolean | null>(null);

  // Update defaults when driver changes
  $effect(() => {
    if (driver === "postgres") {
      port = 5432;
      database = "postgres";
      username = "postgres";
    } else if (driver === "mysql") {
      port = 3306;
      database = "";
      username = "root";
    } else if (driver === "sqlite") {
      filePath = "";
    }
  });

  async function handleTest() {
    testing = true;
    testResult = null;
    try {
      const config: ConnectionConfig = {
        id: "", name, driver, host, port, database, username, password,
        ssl_mode: "prefer", file_path: filePath, folder,
        use_ssh_tunnel: useSshTunnel, ssh_host: sshHost, ssh_port: sshPort,
        ssh_username: sshUsername, ssh_password: sshPassword,
        ssh_private_key: sshPrivateKey, ssh_passphrase: sshPassphrase,
        created_at: ""
      };
      testResult = await connectionStore.testConnection(config);
    } finally {
      testing = false;
    }
  }

  async function handleSave() {
    const config: ConnectionConfig = {
      id: "", name, driver, host, port, database, username, password,
      ssl_mode: "prefer", file_path: filePath, folder,
      use_ssh_tunnel: useSshTunnel, ssh_host: sshHost, ssh_port: sshPort,
      ssh_username: sshUsername, ssh_password: sshPassword,
      ssh_private_key: sshPrivateKey, ssh_passphrase: sshPassphrase,
      created_at: ""
    };
    await connectionStore.saveConnection(config);
    uiStore.showConnectionModal = false;
    reset();
  }

  function reset() {
    driver = "postgres";
    name = "";
    host = "localhost";
    port = 5432;
    database = "postgres";
    username = "postgres";
    password = "";
    filePath = "";
    folder = "";
    useSshTunnel = false;
    sshHost = "";
    sshPort = 22;
    sshUsername = "";
    sshPassword = "";
    sshPrivateKey = "";
    sshPassphrase = "";
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
      <!-- Driver selector -->
      <div class="flex gap-2">
        <button
          onclick={() => driver = "postgres"}
          class="flex-1 py-1.5 text-[11px] font-medium rounded border {driver === 'postgres' ? 'bg-[#00d4ff]/10 border-[#00d4ff] text-[#00d4ff]' : 'bg-[#1a1a1a] border-[#2a2a2a] text-[#a0a0a0]'}">
          PostgreSQL
        </button>
        <button
          onclick={() => driver = "mysql"}
          class="flex-1 py-1.5 text-[11px] font-medium rounded border {driver === 'mysql' ? 'bg-[#00d4ff]/10 border-[#00d4ff] text-[#00d4ff]' : 'bg-[#1a1a1a] border-[#2a2a2a] text-[#a0a0a0]'}">
          MySQL
        </button>
        <button
          onclick={() => driver = "sqlite"}
          class="flex-1 py-1.5 text-[11px] font-medium rounded border {driver === 'sqlite' ? 'bg-[#00d4ff]/10 border-[#00d4ff] text-[#00d4ff]' : 'bg-[#1a1a1a] border-[#2a2a2a] text-[#a0a0a0]'}">
          SQLite
        </button>
      </div>

      <div>
        <label for="conn-name" class="block text-[11px] font-medium text-[#a0a0a0] mb-1">Name</label>
        <input id="conn-name" type="text" bind:value={name} class="w-full bg-[#1a1a1a] border border-[#2a2a2a] rounded px-3 py-1.5 text-[13px] text-[#e8e8e8] focus:border-[#00d4ff] focus:outline-none" placeholder={driver === "sqlite" ? "My SQLite DB" : "My Database"} />
      </div>

      <div>
        <label for="conn-folder" class="block text-[11px] font-medium text-[#a0a0a0] mb-1">Folder (optional)</label>
        <input id="conn-folder" type="text" bind:value={folder} class="w-full bg-[#1a1a1a] border border-[#2a2a2a] rounded px-3 py-1.5 text-[13px] text-[#e8e8e8] focus:border-[#00d4ff] focus:outline-none" placeholder="Production, Staging, etc." />
      </div>

      {#if driver === "sqlite"}
        <div>
          <label for="conn-file" class="block text-[11px] font-medium text-[#a0a0a0] mb-1">Database File Path</label>
          <input id="conn-file" type="text" bind:value={filePath} class="w-full bg-[#1a1a1a] border border-[#2a2a2a] rounded px-3 py-1.5 text-[13px] text-[#e8e8e8] focus:border-[#00d4ff] focus:outline-none font-mono" placeholder="/path/to/database.db" />
          <p class="text-[10px] text-[#4a4a4a] mt-1">Enter the full path to your .db or .sqlite file</p>
        </div>
      {:else}
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
      {/if}

      {#if driver !== "sqlite"}
        <!-- SSH Tunnel -->
        <div class="pt-2 border-t border-[#2a2a2a]">
          <label class="flex items-center gap-2 cursor-pointer">
            <input type="checkbox" bind:checked={useSshTunnel} class="w-3.5 h-3.5 rounded border-[#2a2a2a] bg-[#1a1a1a] text-[#00d4ff] focus:ring-[#00d4ff]" />
            <span class="text-[11px] font-medium text-[#a0a0a0]">Use SSH Tunnel</span>
          </label>
          {#if useSshTunnel}
            <div class="mt-2 space-y-2 pl-5 border-l-2 border-[#2a2a2a]">
              <div class="grid grid-cols-3 gap-2">
                <div class="col-span-2">
                  <label for="ssh-host" class="block text-[11px] font-medium text-[#a0a0a0] mb-1">SSH Host</label>
                  <input id="ssh-host" type="text" bind:value={sshHost} class="w-full bg-[#1a1a1a] border border-[#2a2a2a] rounded px-3 py-1.5 text-[13px] text-[#e8e8e8] focus:border-[#00d4ff] focus:outline-none" placeholder="bastion.example.com" />
                </div>
                <div>
                  <label for="ssh-port" class="block text-[11px] font-medium text-[#a0a0a0] mb-1">SSH Port</label>
                  <input id="ssh-port" type="number" bind:value={sshPort} class="w-full bg-[#1a1a1a] border border-[#2a2a2a] rounded px-3 py-1.5 text-[13px] text-[#e8e8e8] focus:border-[#00d4ff] focus:outline-none" />
                </div>
              </div>
              <div>
                <label for="ssh-user" class="block text-[11px] font-medium text-[#a0a0a0] mb-1">SSH Username</label>
                <input id="ssh-user" type="text" bind:value={sshUsername} class="w-full bg-[#1a1a1a] border border-[#2a2a2a] rounded px-3 py-1.5 text-[13px] text-[#e8e8e8] focus:border-[#00d4ff] focus:outline-none" />
              </div>
              <div>
                <label for="ssh-pass" class="block text-[11px] font-medium text-[#a0a0a0] mb-1">SSH Password (or use key below)</label>
                <input id="ssh-pass" type="password" bind:value={sshPassword} class="w-full bg-[#1a1a1a] border border-[#2a2a2a] rounded px-3 py-1.5 text-[13px] text-[#e8e8e8] focus:border-[#00d4ff] focus:outline-none" />
              </div>
              <div>
                <label for="ssh-key" class="block text-[11px] font-medium text-[#a0a0a0] mb-1">Private Key Path</label>
                <input id="ssh-key" type="text" bind:value={sshPrivateKey} class="w-full bg-[#1a1a1a] border border-[#2a2a2a] rounded px-3 py-1.5 text-[13px] text-[#e8e8e8] focus:border-[#00d4ff] focus:outline-none font-mono" placeholder="~/.ssh/id_rsa" />
              </div>
              <div>
                <label for="ssh-phrase" class="block text-[11px] font-medium text-[#a0a0a0] mb-1">Key Passphrase (optional)</label>
                <input id="ssh-phrase" type="password" bind:value={sshPassphrase} class="w-full bg-[#1a1a1a] border border-[#2a2a2a] rounded px-3 py-1.5 text-[13px] text-[#e8e8e8] focus:border-[#00d4ff] focus:outline-none" />
              </div>
            </div>
          {/if}
        </div>
      {/if}

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
