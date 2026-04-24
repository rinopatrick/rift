use tokio::task;
use crate::db::query::{QueryResult, ColumnInfo};
use crate::schema::{SchemaInfo, TableInfo, ColumnInfo as SchemaColumn};

#[derive(Clone)]
pub enum DriverWrapper {
    Postgres(deadpool_postgres::Pool),
    Sqlite(String),
}

impl DriverWrapper {
    pub async fn execute(&self, sql: &str) -> Result<QueryResult, String> {
        let start = std::time::Instant::now();
        match self {
            DriverWrapper::Postgres(pool) => {
                let client = pool.get().await.map_err(|e| e.to_string())?;
                crate::db::query::execute_query(&client, sql).await.map_err(|e| e.to_string())
            }
            DriverWrapper::Sqlite(path) => {
                let path = path.clone();
                let sql = sql.to_string();
                task::spawn_blocking(move || {
                    let conn = rusqlite::Connection::open(&path).map_err(|e| e.to_string())?;
                    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
                    let col_count = stmt.column_count();
                    let col_names = stmt.column_names();
                    let cols: Vec<ColumnInfo> = col_names
                        .iter()
                        .map(|name| ColumnInfo {
                            name: name.to_string(),
                            data_type: "TEXT".to_string(),
                        })
                        .collect();
                    let mut result_rows = Vec::new();
                    let mut rows = stmt.query([]).map_err(|e| e.to_string())?;
                    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
                        let mut cells = Vec::new();
                        for i in 0..cols.len() {
                            let val: Option<String> = row.get(i).ok();
                            cells.push(val);
                        }
                        result_rows.push(cells);
                    }
                    Ok(QueryResult {
                        row_count: result_rows.len(),
                        columns: cols,
                        rows: result_rows,
                        execution_time_ms: start.elapsed().as_millis() as u64,
                    })
                })
                .await
                .map_err(|e| e.to_string())?
            }
        }
    }

    pub async fn get_schema(&self) -> Result<Vec<SchemaInfo>, String> {
        match self {
            DriverWrapper::Postgres(pool) => {
                let client = pool.get().await.map_err(|e| e.to_string())?;
                crate::schema::get_schemas(&client).await.map_err(|e| e.to_string())
            }
            DriverWrapper::Sqlite(path) => {
                let path = path.clone();
                task::spawn_blocking(move || {
                    let conn = rusqlite::Connection::open(&path).map_err(|e| e.to_string())?;
                    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name").map_err(|e| e.to_string())?;
                    let mut rows = stmt.query([]).map_err(|e| e.to_string())?;
                    let mut tables = Vec::new();
                    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
                        let name: String = row.get(0).map_err(|e| e.to_string())?;
                        tables.push(name);
                    }
                    let mut table_infos = Vec::new();
                    for table in tables {
                        let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table)).map_err(|e| e.to_string())?;
                        let mut rows = stmt.query([]).map_err(|e| e.to_string())?;
                        let mut columns = Vec::new();
                        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
                            columns.push(SchemaColumn {
                                name: row.get(1).unwrap_or_default(),
                                data_type: row.get(2).unwrap_or_else(|_| "TEXT".to_string()),
                                is_nullable: row.get::<_, i32>(3).unwrap_or(1) != 0,
                                column_default: row.get(4).ok(),
                                is_primary_key: row.get::<_, i32>(5).unwrap_or(0) != 0,
                            });
                        }
                        table_infos.push(TableInfo {
                            schema: "main".to_string(),
                            name: table,
                            columns,
                        });
                    }
                    Ok(vec![SchemaInfo {
                        name: "main".to_string(),
                        tables: table_infos,
                    }])
                })
                .await
                .map_err(|e| e.to_string())?
            }
        }
    }
}
