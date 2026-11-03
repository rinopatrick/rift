use serde::Serialize;
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

pub const DEFAULT_ROW_LIMIT: usize = 5000;
pub const MAX_ROW_LIMIT: usize = 50000;

pub fn clamp_limit(limit: usize) -> usize {
    limit.clamp(1, MAX_ROW_LIMIT)
}

pub async fn execute_query_with_limit(
    client: &tokio_postgres::Client,
    sql: &str,
    limit: usize,
) -> Result<QueryResult, tokio_postgres::Error> {
    let start = std::time::Instant::now();
    let limit = clamp_limit(limit);
    let limited_sql = apply_row_limit(sql, limit);
    let rows = client.query(limited_sql.as_str(), &[]).await?;
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

pub fn apply_row_limit(sql: &str, limit: usize) -> String {
    let trimmed = sql.trim_end().trim_end_matches(';').trim_end();
    let upper = trimmed.to_uppercase();
    if upper.contains("LIMIT") {
        return trimmed.to_string();
    }
    format!("{trimmed} LIMIT {limit}")
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

pub async fn execute_query(
    client: &tokio_postgres::Client,
    sql: &str,
) -> Result<QueryResult, tokio_postgres::Error> {
    execute_query_with_limit(client, sql, DEFAULT_ROW_LIMIT).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_existing_limit() {
        assert_eq!(
            apply_row_limit("SELECT * FROM t LIMIT 10", 5000),
            "SELECT * FROM t LIMIT 10"
        );
    }

    #[test]
    fn appends_limit_without_one() {
        assert_eq!(
            apply_row_limit("SELECT * FROM t", 100),
            "SELECT * FROM t LIMIT 100"
        );
    }

    #[test]
    fn strips_trailing_semicolon() {
        assert_eq!(
            apply_row_limit("SELECT * FROM t;", 100),
            "SELECT * FROM t LIMIT 100"
        );
    }

    #[test]
    fn detects_limit_case_insensitively() {
        assert_eq!(
            apply_row_limit("select * from t limit 5", 5000),
            "select * from t limit 5"
        );
    }

    #[test]
    fn clamps_limit_bounds() {
        assert_eq!(clamp_limit(0), 1);
        assert_eq!(clamp_limit(100), 100);
        assert_eq!(clamp_limit(usize::MAX), MAX_ROW_LIMIT);
    }
}
