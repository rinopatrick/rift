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
