import { invoke } from "@tauri-apps/api/core";
import type { ConnectionConfig, ConnectionInfo } from "../types";

export function createConnectionStore() {
  let connections = $state<ConnectionInfo[]>([]);
  let activeConnectionId = $state<string | null>(null);
  let isConnecting = $state(false);

  async function loadConnections() {
    connections = await invoke<ConnectionInfo[]>("get_connections");
  }

  async function saveConnection(config: ConnectionConfig) {
    const saved = await invoke<ConnectionInfo>("save_connection", { config });
    await loadConnections();
    return saved;
  }

  async function deleteConnection(id: string) {
    await invoke("delete_connection", { id });
    await loadConnections();
    if (activeConnectionId === id) {
      activeConnectionId = null;
    }
  }

  async function testConnection(config: ConnectionConfig) {
    return await invoke<boolean>("test_connection", { config });
  }

  async function connect(id: string) {
    isConnecting = true;
    try {
      await invoke("connect_to_database", { id });
      activeConnectionId = id;
    } finally {
      isConnecting = false;
    }
  }

  return {
    get connections() { return connections; },
    get activeConnectionId() { return activeConnectionId; },
    get isConnecting() { return isConnecting; },
    loadConnections,
    saveConnection,
    deleteConnection,
    testConnection,
    connect,
  };
}

export const connectionStore = createConnectionStore();
