use tauri::State;
use crate::AppState;
use crate::db::connection::{ConnectionConfig, ConnectionInfo};
use crate::db::driver::DriverWrapper;
use crate::db::pool::create_pool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;

pub struct ConnectionPools(pub Arc<AsyncMutex<HashMap<String, DriverWrapper>>>);

impl ConnectionPools {
    pub fn new() -> Self {
        Self(Arc::new(AsyncMutex::new(HashMap::new())))
    }
}

pub struct ActiveQueries(pub Arc<AsyncMutex<HashMap<String, tokio::task::AbortHandle>>>);

impl ActiveQueries {
    pub fn new() -> Self {
        Self(Arc::new(AsyncMutex::new(HashMap::new())))
    }
}

#[derive(serde::Serialize)]
pub struct ImportResult {
    pub row_count: usize,
    pub table_name: String,
    pub columns: Vec<String>,
}

#[tauri::command]
pub async fn test_connection(config: ConnectionConfig) -> Result<bool, String> {
    if config.driver == "sqlite" {
        let _ = rusqlite::Connection::open(&config.file_path).map_err(|e| e.to_string())?;
        Ok(true)
    } else {
        let pool = create_pool(&config).map_err(|e| e.to_string())?;
        let client = pool.get().await.map_err(|e| e.to_string())?;
        client.execute("SELECT 1", &[]).await.map_err(|e| e.to_string())?;
        Ok(true)
    }
}

#[tauri::command]
pub fn save_connection(state: State<AppState>, config: ConnectionConfig) -> Result<ConnectionInfo, String> {
    let state = state.0.lock().map_err(|e| e.to_string())?;
    state.save_connection(&config).map_err(|e| e.to_string())?;
    Ok(config.into())
}

#[tauri::command]
pub fn get_connections(state: State<AppState>) -> Result<Vec<ConnectionInfo>, String> {
    let state = state.0.lock().map_err(|e| e.to_string())?;
    state.get_connections().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_connection(state: State<AppState>, id: String) -> Result<(), String> {
    let state = state.0.lock().map_err(|e| e.to_string())?;
    state.delete_connection(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn connect_to_database(
    state: State<'_, AppState>,
    pools: State<'_, ConnectionPools>,
    id: String,
) -> Result<bool, String> {
    let config = {
        let state = state.0.lock().map_err(|e| e.to_string())?;
        state.get_connection_config(&id).map_err(|e| e.to_string())?
            .ok_or_else(|| "Connection not found".to_string())?
    };

    let driver = if config.driver == "sqlite" {
        DriverWrapper::Sqlite(config.file_path.clone())
    } else {
        let pool = create_pool(&config).map_err(|e| e.to_string())?;
        let client = pool.get().await.map_err(|e| e.to_string())?;
        client.execute("SELECT 1", &[]).await.map_err(|e| e.to_string())?;
        DriverWrapper::Postgres(pool)
    };

    let mut pools = pools.0.lock().await;
    pools.insert(id, driver);
    Ok(true)
}

#[tauri::command]
pub async fn execute_sql(
    pools: State<'_, ConnectionPools>,
    active_queries: State<'_, ActiveQueries>,
    connection_id: String,
    sql: String,
    query_id: String,
) -> Result<crate::db::query::QueryResult, String> {
    // Clone driver and release pool lock so other operations aren't blocked
    let driver = {
        let pools = pools.0.lock().await;
        match pools.get(&connection_id).ok_or("Not connected")? {
            DriverWrapper::Postgres(pool) => DriverWrapper::Postgres(pool.clone()),
            DriverWrapper::Sqlite(path) => DriverWrapper::Sqlite(path.clone()),
        }
    };

    let handle = tokio::spawn(async move {
        driver.execute(&sql).await
    });

    {
        let mut aq = active_queries.0.lock().await;
        aq.insert(query_id.clone(), handle.abort_handle());
    }

    let result = handle.await;

    {
        let mut aq = active_queries.0.lock().await;
        aq.remove(&query_id);
    }

    match result {
        Ok(Ok(r)) => Ok(r),
        Ok(Err(e)) => Err(e),
        Err(e) if e.is_cancelled() => Err("Query cancelled by user".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn cancel_query(
    active_queries: State<'_, ActiveQueries>,
    query_id: String,
) -> Result<(), String> {
    let mut aq = active_queries.0.lock().await;
    if let Some(handle) = aq.remove(&query_id) {
        handle.abort();
    }
    Ok(())
}

#[tauri::command]
pub async fn get_schema(
    pools: State<'_, ConnectionPools>,
    connection_id: String,
) -> Result<Vec<crate::schema::SchemaInfo>, String> {
    let pools = pools.0.lock().await;
    let driver = pools.get(&connection_id).ok_or("Not connected")?;
    driver.get_schema().await
}

#[tauri::command]
pub fn save_query_history(
    state: State<AppState>,
    connection_id: String,
    query: String,
) -> Result<(), String> {
    let state = state.0.lock().map_err(|e| e.to_string())?;
    state.save_query_history(&connection_id, &query).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_query_history(
    state: State<AppState>,
    connection_id: String,
) -> Result<Vec<crate::state::QueryHistoryItem>, String> {
    let state = state.0.lock().map_err(|e| e.to_string())?;
    state.get_query_history(&connection_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_bookmark(
    state: State<AppState>,
    connection_id: String,
    name: String,
    query: String,
) -> Result<String, String> {
    let state = state.0.lock().map_err(|e| e.to_string())?;
    state.save_bookmark(&connection_id, &name, &query).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_bookmarks(
    state: State<AppState>,
    connection_id: String,
) -> Result<Vec<crate::state::QueryBookmark>, String> {
    let state = state.0.lock().map_err(|e| e.to_string())?;
    state.get_bookmarks(&connection_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_bookmark(state: State<AppState>, id: String) -> Result<(), String> {
    let state = state.0.lock().map_err(|e| e.to_string())?;
    state.delete_bookmark(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_cell(
    pools: State<'_, ConnectionPools>,
    connection_id: String,
    schema: String,
    table: String,
    column: String,
    value: Option<String>,
    pk_column: String,
    pk_value: String,
) -> Result<(), String> {
    let pools = pools.0.lock().await;
    let driver = pools.get(&connection_id).ok_or("Not connected")?;
    driver.update_cell(&schema, &table, &column, value.as_deref(), &pk_column, &pk_value).await
}

#[tauri::command]
pub async fn export_csv(
    pools: State<'_, ConnectionPools>,
    connection_id: String,
    sql: String,
) -> Result<String, String> {
    let pools = pools.0.lock().await;
    let driver = pools.get(&connection_id).ok_or("Not connected")?;
    let result = driver.execute(&sql).await?;

    let mut csv = String::new();
    csv.push_str(&result.columns.iter().map(|c| c.name.clone()).collect::<Vec<_>>().join(","));
    csv.push('\n');

    for row in &result.rows {
        let cells: Vec<String> = row.iter().map(|c| {
            match c {
                Some(v) => format!("\"{}\"", v.replace('\"', "\"\"")),
                None => "\"\"".to_string(),
            }
        }).collect();
        csv.push_str(&cells.join(","));
        csv.push('\n');
    }

    Ok(csv)
}

#[tauri::command]
pub async fn export_json(
    pools: State<'_, ConnectionPools>,
    connection_id: String,
    sql: String,
) -> Result<String, String> {
    let pools = pools.0.lock().await;
    let driver = pools.get(&connection_id).ok_or("Not connected")?;
    let result = driver.execute(&sql).await?;

    let mut objects = Vec::new();
    for row in &result.rows {
        let mut obj = serde_json::Map::new();
        for (i, col) in result.columns.iter().enumerate() {
            let value = match &row[i] {
                Some(v) => serde_json::Value::String(v.clone()),
                None => serde_json::Value::Null,
            };
            obj.insert(col.name.clone(), value);
        }
        objects.push(serde_json::Value::Object(obj));
    }

    serde_json::to_string_pretty(&objects).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn import_csv(
    pools: State<'_, ConnectionPools>,
    connection_id: String,
    table_name: String,
    csv_content: String,
) -> Result<ImportResult, String> {
    let pools = pools.0.lock().await;
    let driver = pools.get(&connection_id).ok_or("Not connected")?;

    // Parse CSV
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_content.as_bytes());

    let headers: Vec<String> = reader
        .headers()
        .map_err(|e| e.to_string())?
        .iter()
        .map(|h| sanitize_column_name(h))
        .collect();

    if headers.is_empty() {
        return Err("CSV has no headers".to_string());
    }

    // Collect all rows
    let mut rows: Vec<Vec<String>> = Vec::new();
    for result in reader.records() {
        let record = result.map_err(|e| e.to_string())?;
        rows.push(record.iter().map(|s| s.to_string()).collect());
    }

    if rows.is_empty() {
        return Err("CSV has no data rows".to_string());
    }

    // Detect types from first 1000 rows
    let sample_size = rows.len().min(1000);
    let mut col_types: Vec<&str> = Vec::with_capacity(headers.len());
    for col_idx in 0..headers.len() {
        let mut detected = "TEXT";
        for row in &rows[..sample_size] {
            let val = row.get(col_idx).map(|s| s.trim()).unwrap_or("");
            if val.is_empty() {
                continue;
            }
            let t = detect_type(val);
            if detected == "TEXT" || (detected == "TIMESTAMP" && t != "TEXT") || (detected == "BOOLEAN" && (t == "INTEGER" || t == "REAL")) || (detected == "INTEGER" && t == "REAL") {
                detected = t;
            }
        }
        col_types.push(detected);
    }

    // Determine driver type for type mapping
    let is_sqlite = matches!(driver, DriverWrapper::Sqlite(_));

    // Build CREATE TABLE
    let create_cols: Vec<String> = headers.iter().enumerate().map(|(i, name)| {
        let sql_type = if is_sqlite {
            match col_types[i] {
                "INTEGER" => "INTEGER",
                "REAL" => "REAL",
                "BOOLEAN" => "INTEGER",
                "TIMESTAMP" => "TEXT",
                _ => "TEXT",
            }
        } else {
            match col_types[i] {
                "INTEGER" => "INTEGER",
                "REAL" => "NUMERIC",
                "BOOLEAN" => "BOOLEAN",
                "TIMESTAMP" => "TIMESTAMP",
                _ => "TEXT",
            }
        };
        format!("\"{}\" {}", name, sql_type)
    }).collect();

    let create_sql = format!("CREATE TABLE IF NOT EXISTS \"{}\" ({})", table_name, create_cols.join(", "));
    driver.execute(&create_sql).await?;

    // Batch INSERT
    const BATCH_SIZE: usize = 500;
    for chunk in rows.chunks(BATCH_SIZE) {
        let mut values_parts = Vec::new();
        for row in chunk {
            let vals: Vec<String> = row.iter().map(|v| {
                if v.is_empty() {
                    "NULL".to_string()
                } else {
                    format!("'{}'", v.replace('\'', "''"))
                }
            }).collect();
            values_parts.push(format!("({})", vals.join(", ")));
        }
        let insert_sql = format!(
            "INSERT INTO \"{}\" ({}) VALUES {}",
            table_name,
            headers.iter().map(|h| format!("\"{}\"", h)).collect::<Vec<_>>().join(", "),
            values_parts.join(", ")
        );
        driver.execute(&insert_sql).await?;
    }

    Ok(ImportResult {
        row_count: rows.len(),
        table_name,
        columns: headers,
    })
}

fn sanitize_column_name(name: &str) -> String {
    name.trim()
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != '_', "_")
        .replace("__", "_")
        .trim_start_matches('_')
        .trim_end_matches('_')
        .to_string()
}

fn detect_type(val: &str) -> &'static str {
    if val.parse::<i64>().is_ok() {
        return "INTEGER";
    }
    if val.parse::<f64>().is_ok() {
        return "REAL";
    }
    let lower = val.to_lowercase();
    if lower == "true" || lower == "false" || lower == "1" || lower == "0" || lower == "yes" || lower == "no" || lower == "t" || lower == "f" {
        return "BOOLEAN";
    }
    // Simple ISO 8601 or common date formats
    if val.len() >= 10 {
        let date_like = val.contains('-') && (val.contains('T') || val.contains(' '));
        if date_like {
            return "TIMESTAMP";
        }
    }
    "TEXT"
}

#[tauri::command]
pub fn save_setting(state: State<AppState>, key: String, value: String) -> Result<(), String> {
    let state = state.0.lock().map_err(|e| e.to_string())?;
    state.save_setting(&key, &value).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_settings(state: State<AppState>) -> Result<Vec<(String, String)>, String> {
    let state = state.0.lock().map_err(|e| e.to_string())?;
    state.get_all_settings().map_err(|e| e.to_string())
}
