use tokio::task;
use mysql_async::prelude::Queryable;
use crate::db::query::{QueryResult, ColumnInfo};
use crate::schema::{SchemaInfo, TableInfo, ColumnInfo as SchemaColumn};

#[derive(Clone)]
pub enum DriverWrapper {
    Postgres(deadpool_postgres::Pool),
    Mysql(mysql_async::Pool),
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
            DriverWrapper::Mysql(pool) => {
                let mut conn = pool.get_conn().await.map_err(|e| e.to_string())?;
                let mut result = conn.query_iter(sql).await.map_err(|e| e.to_string())?;
                let cols: Vec<ColumnInfo> = result.columns_ref()
                    .iter()
                    .map(|c| ColumnInfo {
                        name: c.name_str().to_string(),
                        data_type: format!("{:?}", c.column_type()),
                    })
                    .collect();
                let mut rows = Vec::new();
                while let Ok(Some(row)) = result.next().await {
                    let mut cells = Vec::new();
                    for i in 0..cols.len() {
                        let val: Option<String> = row.get(i);
                        cells.push(val);
                    }
                    rows.push(cells);
                }
                Ok(QueryResult {
                    row_count: rows.len(),
                    columns: cols,
                    rows,
                    execution_time_ms: start.elapsed().as_millis() as u64,
                })
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
            DriverWrapper::Mysql(pool) => {
                let mut conn = pool.get_conn().await.map_err(|e| e.to_string())?;
                // Get current database
                let db_result: Vec<mysql_async::Row> = conn.query("SELECT DATABASE()").await.map_err(|e| e.to_string())?;
                let db_name: String = if let Some(row) = db_result.first() {
                    row.get(0).unwrap_or_default()
                } else {
                    return Err("Could not determine current database".to_string());
                };

                let tables_sql = format!(
                    "SELECT table_name FROM information_schema.tables WHERE table_schema = '{}' AND table_type = 'BASE TABLE' ORDER BY table_name",
                    db_name.replace('\'', "''")
                );
                let table_rows: Vec<mysql_async::Row> = conn.query(&tables_sql).await.map_err(|e| e.to_string())?;
                let mut tables = Vec::new();

                for table_row in table_rows {
                    let table_name: String = table_row.get(0).unwrap_or_default();
                    let cols_sql = format!(
                        "SELECT column_name, data_type, is_nullable, column_default, column_key FROM information_schema.columns WHERE table_schema = '{}' AND table_name = '{}' ORDER BY ordinal_position",
                        db_name.replace('\'', "''"),
                        table_name.replace('\'', "''")
                    );
                    let col_rows: Vec<mysql_async::Row> = conn.query(&cols_sql).await.map_err(|e| e.to_string())?;
                    let mut columns = Vec::new();
                    for col_row in col_rows {
                        let col_name: String = col_row.get(0).unwrap_or_default();
                        let data_type: String = col_row.get(1).unwrap_or_else(|| "TEXT".to_string());
                        let is_nullable: String = col_row.get(2).unwrap_or_else(|| "YES".to_string());
                        let column_default: Option<String> = col_row.get(3);
                        let column_key: String = col_row.get(4).unwrap_or_default();
                        columns.push(SchemaColumn {
                            name: col_name,
                            data_type,
                            is_nullable: is_nullable == "YES",
                            column_default,
                            is_primary_key: column_key == "PRI",
                        });
                    }
                    tables.push(TableInfo {
                        schema: db_name.clone(),
                        name: table_name,
                        columns,
                    });
                }

                Ok(vec![SchemaInfo {
                    name: db_name,
                    tables,
                }])
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

    pub async fn get_foreign_keys(&self) -> Result<Vec<crate::schema::ForeignKeyInfo>, String> {
        match self {
            DriverWrapper::Postgres(pool) => {
                let client = pool.get().await.map_err(|e| e.to_string())?;
                crate::schema::get_foreign_keys(&client).await.map_err(|e| e.to_string())
            }
            DriverWrapper::Mysql(pool) => {
                let mut conn = pool.get_conn().await.map_err(|e| e.to_string())?;
                let db_result: Vec<mysql_async::Row> = conn.query("SELECT DATABASE()").await.map_err(|e| e.to_string())?;
                let db_name: String = if let Some(row) = db_result.first() {
                    row.get(0).unwrap_or_default()
                } else {
                    return Ok(Vec::new());
                };
                let sql = format!(
                    "SELECT constraint_name, table_name, column_name, referenced_table_name, referenced_column_name FROM information_schema.key_column_usage WHERE table_schema = '{}' AND referenced_table_name IS NOT NULL",
                    db_name.replace('\'', "''")
                );
                let rows: Vec<mysql_async::Row> = conn.query(&sql).await.map_err(|e| e.to_string())?;
                Ok(rows.iter().map(|r| crate::schema::ForeignKeyInfo {
                    constraint_name: r.get(0).unwrap_or_default(),
                    table_schema: db_name.clone(),
                    table_name: r.get(1).unwrap_or_default(),
                    column_name: r.get(2).unwrap_or_default(),
                    foreign_table_schema: db_name.clone(),
                    foreign_table_name: r.get(3).unwrap_or_default(),
                    foreign_column_name: r.get(4).unwrap_or_default(),
                }).collect())
            }
            DriverWrapper::Sqlite(_) => {
                Err("Foreign key visualization not yet supported for SQLite".to_string())
            }
        }
    }

    pub async fn update_cell(
        &self,
        schema: &str,
        table: &str,
        column: &str,
        value: Option<&str>,
        pk_column: &str,
        pk_value: &str,
    ) -> Result<(), String> {
        match self {
            DriverWrapper::Postgres(pool) => {
                let client = pool.get().await.map_err(|e| e.to_string())?;
                let sql = format!(
                    r#"UPDATE "{}"."{}" SET "{}" = $1 WHERE "{}" = $2"#,
                    schema, table, column, pk_column
                );
                client.execute(&sql, &[&value, &pk_value]).await.map_err(|e| e.to_string())?;
                Ok(())
            }
            DriverWrapper::Mysql(pool) => {
                let mut conn = pool.get_conn().await.map_err(|e| e.to_string())?;
                let sql = format!(
                    "UPDATE `{}`.`{}` SET `{}` = ? WHERE `{}` = ?",
                    schema, table, column, pk_column
                );
                conn.exec_drop(&sql, (value, pk_value)).await.map_err(|e| e.to_string())?;
                Ok(())
            }
            DriverWrapper::Sqlite(path) => {
                let path = path.clone();
                let _schema = schema.to_string();
                let table = table.to_string();
                let column = column.to_string();
                let value = value.map(|s| s.to_string());
                let pk_column = pk_column.to_string();
                let pk_value = pk_value.to_string();
                task::spawn_blocking(move || {
                    let conn = rusqlite::Connection::open(&path).map_err(|e| e.to_string())?;
                    let sql = format!(
                        r#"UPDATE "{}" SET "{}" = ?1 WHERE "{}" = ?2"#,
                        table, column, pk_column
                    );
                    conn.execute(&sql, rusqlite::params![value.as_deref(), pk_value.as_str()]).map_err(|e| e.to_string())?;
                    Ok(())
                })
                .await
                .map_err(|e| e.to_string())?
            }
        }
    }
}
