use serde::Serialize;
use tokio_postgres::Client;

#[derive(Debug, Clone, Serialize)]
pub struct SchemaDiff {
    pub added_tables: Vec<TableDiff>,
    pub removed_tables: Vec<TableDiff>,
    pub modified_tables: Vec<TableModification>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TableDiff {
    pub name: String,
    pub columns: Vec<ColumnDiff>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ColumnDiff {
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TableModification {
    pub name: String,
    pub added_columns: Vec<ColumnDiff>,
    pub removed_columns: Vec<ColumnDiff>,
    pub modified_columns: Vec<ColumnChange>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ColumnChange {
    pub name: String,
    pub old_type: String,
    pub new_type: String,
    pub old_nullable: bool,
    pub new_nullable: bool,
}

async fn fetch_postgres_schema(client: &Client) -> Result<Vec<TableDiff>, String> {
    let rows = client
        .query(
            "SELECT table_name, column_name, data_type, is_nullable, column_default
             FROM information_schema.columns
             WHERE table_schema = 'public'
             ORDER BY table_name, ordinal_position",
            &[],
        )
        .await
        .map_err(|e| e.to_string())?;

    let mut tables: Vec<TableDiff> = Vec::new();
    let mut current_table: Option<&mut TableDiff> = None;

    for row in &rows {
        let table_name: String = row.get(0);
        let col = ColumnDiff {
            name: row.get(1),
            data_type: row.get(2),
            is_nullable: row.get::<_, String>(3) == "YES",
            default_value: row.get(4),
        };

        match &mut current_table {
            Some(t) if t.name == table_name => t.columns.push(col),
            _ => {
                tables.push(TableDiff {
                    name: table_name.clone(),
                    columns: vec![col],
                });
                current_table = tables.last_mut();
            }
        }
    }

    Ok(tables)
}

pub fn compute_diff(source: Vec<TableDiff>, target: Vec<TableDiff>) -> SchemaDiff {
    let source_map: std::collections::HashMap<String, TableDiff> =
        source.into_iter().map(|t| (t.name.clone(), t)).collect();
    let target_map: std::collections::HashMap<String, TableDiff> =
        target.into_iter().map(|t| (t.name.clone(), t)).collect();

    let mut added_tables = Vec::new();
    let mut removed_tables = Vec::new();
    let mut modified_tables = Vec::new();

    for (name, table) in &target_map {
        if !source_map.contains_key(name) {
            added_tables.push(table.clone());
        }
    }

    for (name, table) in &source_map {
        if !target_map.contains_key(name) {
            removed_tables.push(table.clone());
        }
    }

    for (name, source_table) in &source_map {
        if let Some(target_table) = target_map.get(name) {
            let source_cols: std::collections::HashMap<String, ColumnDiff> = source_table
                .columns
                .iter()
                .map(|c| (c.name.clone(), c.clone()))
                .collect();
            let target_cols: std::collections::HashMap<String, ColumnDiff> = target_table
                .columns
                .iter()
                .map(|c| (c.name.clone(), c.clone()))
                .collect();

            let mut added_columns = Vec::new();
            let mut removed_columns = Vec::new();
            let mut modified_columns = Vec::new();

            for (cname, col) in &target_cols {
                if !source_cols.contains_key(cname) {
                    added_columns.push(col.clone());
                }
            }

            for (cname, col) in &source_cols {
                if !target_cols.contains_key(cname) {
                    removed_columns.push(col.clone());
                }
            }

            for (cname, source_col) in &source_cols {
                if let Some(target_col) = target_cols.get(cname) {
                    if source_col.data_type != target_col.data_type
                        || source_col.is_nullable != target_col.is_nullable
                    {
                        modified_columns.push(ColumnChange {
                            name: cname.clone(),
                            old_type: source_col.data_type.clone(),
                            new_type: target_col.data_type.clone(),
                            old_nullable: source_col.is_nullable,
                            new_nullable: target_col.is_nullable,
                        });
                    }
                }
            }

            if !added_columns.is_empty()
                || !removed_columns.is_empty()
                || !modified_columns.is_empty()
            {
                modified_tables.push(TableModification {
                    name: name.clone(),
                    added_columns,
                    removed_columns,
                    modified_columns,
                });
            }
        }
    }

    SchemaDiff {
        added_tables,
        removed_tables,
        modified_tables,
    }
}

pub async fn diff_postgres_schemas(
    source_client: &Client,
    target_client: &Client,
) -> Result<SchemaDiff, String> {
    let source = fetch_postgres_schema(source_client).await?;
    let target = fetch_postgres_schema(target_client).await?;
    Ok(compute_diff(source, target))
}
