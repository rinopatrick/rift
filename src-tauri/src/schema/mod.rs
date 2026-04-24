use serde::{Deserialize, Serialize};
use tokio_postgres::Client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaInfo {
    pub name: String,
    pub tables: Vec<TableInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableInfo {
    pub schema: String,
    pub name: String,
    pub columns: Vec<ColumnInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub column_default: Option<String>,
    pub is_primary_key: bool,
}

pub async fn get_schemas(client: &Client) -> Result<Vec<SchemaInfo>, tokio_postgres::Error> {
    let schema_rows = client
        .query(
            "SELECT schema_name FROM information_schema.schemas WHERE schema_name NOT IN ('pg_catalog', 'information_schema', 'pg_toast') ORDER BY schema_name",
            &[],
        )
        .await?;

    let mut schemas = Vec::new();

    for schema_row in schema_rows {
        let schema_name: String = schema_row.get(0);

        let table_rows = client
            .query(
                "SELECT table_name FROM information_schema.tables WHERE table_schema = $1 AND table_type = 'BASE TABLE' ORDER BY table_name",
                &[&schema_name],
            )
            .await?;

        let mut tables = Vec::new();

        for table_row in table_rows {
            let table_name: String = table_row.get(0);

            let col_rows = client
                .query(
                    "SELECT column_name, data_type, is_nullable, column_default FROM information_schema.columns WHERE table_schema = $1 AND table_name = $2 ORDER BY ordinal_position",
                    &[&schema_name, &table_name],
                )
                .await?;

            let pk_rows = client
                .query(
                    "SELECT kcu.column_name FROM information_schema.table_constraints tc JOIN information_schema.key_column_usage kcu ON tc.constraint_name = kcu.constraint_name AND tc.table_schema = kcu.table_schema WHERE tc.constraint_type = 'PRIMARY KEY' AND tc.table_schema = $1 AND tc.table_name = $2",
                    &[&schema_name, &table_name],
                )
                .await?;

            let pk_columns: std::collections::HashSet<String> = pk_rows
                .iter()
                .map(|r| r.get::<_, String>(0))
                .collect();

            let columns: Vec<ColumnInfo> = col_rows
                .iter()
                .map(|r| ColumnInfo {
                    name: r.get(0),
                    data_type: r.get(1),
                    is_nullable: r.get::<_, String>(2) == "YES",
                    column_default: r.get(3),
                    is_primary_key: pk_columns.contains(&r.get::<_, String>(0)),
                })
                .collect();

            tables.push(TableInfo {
                schema: schema_name.clone(),
                name: table_name,
                columns,
            });
        }

        schemas.push(SchemaInfo {
            name: schema_name,
            tables,
        });
    }

    Ok(schemas)
}
