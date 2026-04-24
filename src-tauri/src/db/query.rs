use serde::{Serialize};
use tokio_postgres::Row;

#[derive(Debug, Clone, Serialize)]
pub struct QueryResult {
    pub columns: Vec<ColumnInfo>,
    pub rows: Vec<Vec<Option<String>>>,
    pub row_count: usize,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
}

pub async fn execute_query(
    client: &tokio_postgres::Client,
    sql: &str,
) -> Result<QueryResult, tokio_postgres::Error> {
    let start = std::time::Instant::now();
    
    let rows = client.query(sql, &[]).await?;
    let execution_time_ms = start.elapsed().as_millis() as u64;
    
    if rows.is_empty() {
        return Ok(QueryResult {
            columns: vec![],
            rows: vec![],
            row_count: 0,
            execution_time_ms,
        });
    }
    
    let columns: Vec<ColumnInfo> = rows[0]
        .columns()
        .iter()
        .map(|c| ColumnInfo {
            name: c.name().to_string(),
            data_type: format!("{:?}", c.type_()),
        })
        .collect();
    
    let mut result_rows: Vec<Vec<Option<String>>> = Vec::with_capacity(rows.len());
    for row in &rows {
        let mut result_row = Vec::with_capacity(columns.len());
        for (i, _col) in columns.iter().enumerate() {
            let val = row_to_string(row, i);
            result_row.push(val);
        }
        result_rows.push(result_row);
    }
    
    let row_count = result_rows.len();
    
    Ok(QueryResult {
        columns,
        rows: result_rows,
        row_count,
        execution_time_ms,
    })
}

fn row_to_string(row: &Row, idx: usize) -> Option<String> {
    if let Ok(val) = row.try_get::<_, Option<String>>(idx) {
        return val;
    }
    if let Ok(val) = row.try_get::<_, Option<i32>>(idx) {
        return val.map(|v| v.to_string());
    }
    if let Ok(val) = row.try_get::<_, Option<i64>>(idx) {
        return val.map(|v| v.to_string());
    }
    if let Ok(val) = row.try_get::<_, Option<bool>>(idx) {
        return val.map(|v| v.to_string());
    }
    if let Ok(val) = row.try_get::<_, Option<f64>>(idx) {
        return val.map(|v| v.to_string());
    }
    if let Ok(val) = row.try_get::<_, Option<serde_json::Value>>(idx) {
        return val.map(|v| v.to_string());
    }
    Some("[unsupported]".to_string())
}
