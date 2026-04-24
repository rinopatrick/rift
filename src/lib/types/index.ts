export interface ConnectionConfig {
  id: string;
  name: string;
  host: string;
  port: number;
  database: string;
  username: string;
  password: string;
  ssl_mode: string;
  created_at: string;
}

export interface ConnectionInfo {
  id: string;
  name: string;
  host: string;
  port: number;
  database: string;
  username: string;
  ssl_mode: string;
  created_at: string;
}

export interface ColumnInfo {
  name: string;
  data_type: string;
}

export interface QueryResult {
  columns: ColumnInfo[];
  rows: (string | null)[][];
  row_count: number;
  execution_time_ms: number;
}

export interface QueryTab {
  id: string;
  name: string;
  sql: string;
  result: QueryResult | null;
  status: "idle" | "running" | "error";
  error?: string;
}
