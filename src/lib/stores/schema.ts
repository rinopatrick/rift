import { invoke } from "@tauri-apps/api/core";
import type { SchemaInfo } from "../types";

export function createSchemaStore() {
  let schemas = $state<SchemaInfo[]>([]);
  let expandedSchemas = $state<Set<string>>(new Set());
  let expandedTables = $state<Set<string>>(new Set());
  let loading = $state(false);

  async function loadSchema(connectionId: string) {
    loading = true;
    try {
      schemas = await invoke<SchemaInfo[]>("get_schema", { connectionId });
    } finally {
      loading = false;
    }
  }

  function toggleSchema(name: string) {
    const next = new Set(expandedSchemas);
    if (next.has(name)) next.delete(name);
    else next.add(name);
    expandedSchemas = next;
  }

  function toggleTable(key: string) {
    const next = new Set(expandedTables);
    if (next.has(key)) next.delete(key);
    else next.add(key);
    expandedTables = next;
  }

  function clear() {
    schemas = [];
    expandedSchemas = new Set();
    expandedTables = new Set();
  }

  return {
    get schemas() { return schemas; },
    get expandedSchemas() { return expandedSchemas; },
    get expandedTables() { return expandedTables; },
    get loading() { return loading; },
    loadSchema,
    toggleSchema,
    toggleTable,
    clear,
  };
}

export const schemaStore = createSchemaStore();
