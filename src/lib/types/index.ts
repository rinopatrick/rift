export interface ConnectionConfig {
  id: string;
  name: string;
  driver: string;
  host: string;
  port: number;
  database: string;
  username: string;
  password: string;
  ssl_mode: string;
  file_path: string;
  folder: string;
  use_ssh_tunnel: boolean;
  ssh_host: string;
  ssh_port: number;
  ssh_username: string;
  ssh_password: string;
  ssh_private_key: string;
  ssh_passphrase: string;
  created_at: string;
}

export interface ConnectionInfo {
  id: string;
  name: string;
  driver: string;
  host: string;
  port: number;
  database: string;
  username: string;
  ssl_mode: string;
  file_path: string;
  folder: string;
  use_ssh_tunnel: boolean;
  ssh_host: string;
  ssh_port: number;
  ssh_username: string;
  created_at: string;
}

export interface ColumnInfo {
  name: string;
  data_type: string;
  is_nullable: boolean;
  column_default: string | null;
  is_primary_key: boolean;
}

export interface TableInfo {
  schema: string;
  name: string;
  columns: ColumnInfo[];
}

export interface SchemaInfo {
  name: string;
  tables: TableInfo[];
}

export interface QueryResult {
  columns: { name: string; data_type: string }[];
  rows: (string | null)[][];
  row_count: number;
  execution_time_ms: number;
}

export interface QueryTab {
  id: string;
  name: string;
  sql: string;
  result: QueryResult | null;
  status: "idle" | "running" | "error" | "cancelled";
  error?: string;
  queryId?: string;
  viewMode?: "grid" | "chart";
  explainData?: any;
  explainError?: string;
  explainLoading?: boolean;
}
export interface QueryHistoryItem {
  id: string;
  connection_id: string;
  query: string;
  executed_at: string;
}
