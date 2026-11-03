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

/// Render a `SchemaDiff` as executable PostgreSQL migration SQL that brings
/// the target schema in line with the source schema. Table/column identifiers
/// are validated through `quote_ident` so hostile names fail loudly instead
/// of injecting SQL. Column defaults from `information_schema` pass through
/// only when they look like safe literals (quoted strings, numbers,
/// booleans, NULL, common casts, `now()`/`CURRENT_*`); anything else is
/// dropped to avoid executing arbitrary expressions.
pub fn render_migration_sql(diff: &SchemaDiff) -> Result<String, String> {
    use crate::db::idents::quote_ident;

    /// Validate a PostgreSQL type name from `information_schema` before
    /// interpolating it into generated DDL. Allows plain names,
    /// schema-qualified names, `varchar(n)` / `numeric(p,s)` precision,
    /// and `[]` array suffixes. Anything else fails loudly.
    fn valid_type_name(raw: &str) -> bool {
        let t = raw.trim();
        if t.is_empty() || t.len() > 128 || t.contains(';') {
            return false;
        }
        let mut rest = t;
        if let Some(stripped) = rest.strip_suffix("[]") {
            rest = stripped.trim_end();
        }
        let base = match rest.find('(') {
            Some(open) => {
                let Some(close) = rest.rfind(')') else {
                    return false;
                };
                if close != rest.len() - 1 {
                    return false;
                }
                let params = &rest[open + 1..close];
                if params.split(',').any(|p| {
                    let p = p.trim();
                    p.is_empty() || p.len() > 10 || !p.chars().all(|c| c.is_ascii_digit())
                }) {
                    return false;
                }
                &rest[..open]
            }
            None => rest,
        };
        let dots = base.matches('.').count();
        if dots > 1 {
            return false;
        }
        for part in base.split('.') {
            let part = part.trim();
            if part.is_empty()
                || !part
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_' || c == ' ')
            {
                return false;
            }
        }
        true
    }

    /// Validate a column default from `information_schema` before
    /// interpolating it into generated DDL. The whole string must match
    /// one safe shape, so a hostile default cannot smuggle a second
    /// statement past a prefix check.
    fn safe_default(raw: &str) -> Option<String> {
        fn quoted_literal(s: &str) -> bool {
            if s.len() < 2 || !s.starts_with('\'') || !s.ends_with('\'') {
                return false;
            }
            let inner = &s[1..s.len() - 1];
            if inner.contains(';') || inner.contains("--") || inner.contains("/*") {
                return false;
            }
            // Only `''` escapes allowed; a lone backslash-quote ends the literal.
            let mut chars = inner.chars();
            while let Some(c) = chars.next() {
                if c == '\'' && chars.next() != Some('\'') {
                    return false;
                }
            }
            true
        }

        fn bare_atom(s: &str) -> bool {
            let lower = s.to_lowercase();
            s.parse::<i64>().is_ok()
                || s.parse::<f64>().is_ok()
                || ["true", "false", "null"].contains(&lower.as_str())
        }

        let t = raw.trim();
        if t.is_empty() || t.contains(';') {
            return None;
        }
        let lower = t.to_lowercase();
        // Zero-argument clock functions with nothing trailing.
        if ["now()", "current_timestamp", "current_date", "current_time"].contains(&lower.as_str())
        {
            return Some(t.to_string());
        }
        // Optional single `::type` cast; head must be a full literal or atom.
        let (head, tail) = match t.split_once("::") {
            Some((h, tail)) => (h.trim(), Some(tail.trim())),
            None => (t, None),
        };
        if let Some(ty) = tail {
            if ty.is_empty() || !valid_type_name(ty) {
                return None;
            }
            if !(quoted_literal(head) || bare_atom(head)) {
                return None;
            }
            return Some(t.to_string());
        }
        if quoted_literal(head) || bare_atom(head) {
            return Some(t.to_string());
        }
        None
    }

    fn column_def(col: &ColumnDiff) -> Result<String, String> {
        if !valid_type_name(&col.data_type) {
            return Err(format!("Unsupported column type: {}", col.data_type));
        }
        let mut def = format!("{} {}", quote_ident(&col.name)?, col.data_type.trim());
        if !col.is_nullable {
            def.push_str(" NOT NULL");
        }
        if let Some(d) = col.default_value.as_deref().and_then(safe_default) {
            def.push_str(&format!(" DEFAULT {d}"));
        }
        Ok(def)
    }

    let mut out = vec!["-- Generated by Rift: apply source schema onto target".to_string()];

    for table in &diff.removed_tables {
        out.push(format!("DROP TABLE {};", quote_ident(&table.name)?));
    }

    for table in &diff.added_tables {
        let cols: Vec<String> = table
            .columns
            .iter()
            .map(column_def)
            .collect::<Result<_, _>>()?;
        out.push(format!(
            "CREATE TABLE {} ({});",
            quote_ident(&table.name)?,
            cols.join(", ")
        ));
    }

    for m in &diff.modified_tables {
        let t = quote_ident(&m.name)?;
        for col in &m.removed_columns {
            out.push(format!(
                "ALTER TABLE {} DROP COLUMN {};",
                t,
                quote_ident(&col.name)?
            ));
        }
        for col in &m.added_columns {
            out.push(format!(
                "ALTER TABLE {} ADD COLUMN {};",
                t,
                column_def(col)?
            ));
        }
        for change in &m.modified_columns {
            let c = quote_ident(&change.name)?;
            if !valid_type_name(&change.new_type) {
                return Err(format!("Unsupported column type: {}", change.new_type));
            }
            out.push(format!(
                "ALTER TABLE {} ALTER COLUMN {} TYPE {};",
                t,
                c,
                change.new_type.trim()
            ));
            if change.old_nullable != change.new_nullable {
                if change.new_nullable {
                    out.push(format!("ALTER TABLE {t} ALTER COLUMN {c} DROP NOT NULL;"));
                } else {
                    out.push(format!("ALTER TABLE {t} ALTER COLUMN {c} SET NOT NULL;"));
                }
            }
        }
    }

    if out.len() == 1 {
        out.push("-- No differences -- schemas are identical.".to_string());
    }
    Ok(out.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn col(name: &str, data_type: &str, nullable: bool, default: Option<&str>) -> ColumnDiff {
        ColumnDiff {
            name: name.to_string(),
            data_type: data_type.to_string(),
            is_nullable: nullable,
            default_value: default.map(|s| s.to_string()),
        }
    }

    #[test]
    fn renders_added_table_with_safe_default() {
        let diff = SchemaDiff {
            added_tables: vec![TableDiff {
                name: "users".to_string(),
                columns: vec![
                    col("id", "integer", false, None),
                    col("name", "text", true, Some("'anon'::text")),
                ],
            }],
            removed_tables: vec![],
            modified_tables: vec![],
        };
        let sql = render_migration_sql(&diff).unwrap();
        assert!(sql.contains("CREATE TABLE \"users\""), "{sql}");
        assert!(sql.contains("\"name\" text DEFAULT 'anon'::text"), "{sql}");
    }

    #[test]
    fn drops_unsafe_default_expression() {
        let diff = SchemaDiff {
            added_tables: vec![TableDiff {
                name: "t".to_string(),
                columns: vec![col(
                    "c",
                    "text",
                    true,
                    Some("(SELECT password FROM users LIMIT 1)"),
                )],
            }],
            removed_tables: vec![],
            modified_tables: vec![],
        };
        let sql = render_migration_sql(&diff).unwrap();
        assert!(!sql.contains("SELECT"), "{sql}");
        assert!(sql.contains("\"c\" text"), "{sql}");
    }

    #[test]
    fn drops_quote_breakout_default() {
        for payload in [
            "'; DROP TABLE users; --'",
            "now()); DROP TABLE users; --",
            "null::text; DROP TABLE users; --",
            "'a'; DROP TABLE x; --'::text",
            "'it''s; DROP TABLE users; --'",
        ] {
            let diff = SchemaDiff {
                added_tables: vec![TableDiff {
                    name: "t".to_string(),
                    columns: vec![col("c", "text", true, Some(payload))],
                }],
                removed_tables: vec![],
                modified_tables: vec![],
            };
            let sql = render_migration_sql(&diff).unwrap();
            assert!(!sql.contains("DROP TABLE users"), "{payload} leaked: {sql}");
            assert!(!sql.contains("DROP TABLE x"), "{payload} leaked: {sql}");
            assert!(!sql.contains("DEFAULT"), "{payload} leaked: {sql}");
        }
    }

    #[test]
    fn keeps_escaped_quote_literal() {
        let diff = SchemaDiff {
            added_tables: vec![TableDiff {
                name: "t".to_string(),
                columns: vec![col("c", "text", true, Some("'it''s'"))],
            }],
            removed_tables: vec![],
            modified_tables: vec![],
        };
        let sql = render_migration_sql(&diff).unwrap();
        assert!(sql.contains("DEFAULT 'it''s'"), "{sql}");
    }

    #[test]
    fn rejects_hostile_type_names() {
        for ty in ["text); DROP TABLE users; --", "text; DROP TABLE users", ""] {
            let diff = SchemaDiff {
                added_tables: vec![TableDiff {
                    name: "t".to_string(),
                    columns: vec![col("c", ty, true, None)],
                }],
                removed_tables: vec![],
                modified_tables: vec![],
            };
            assert!(render_migration_sql(&diff).is_err(), "{ty} accepted");
        }
    }

    #[test]
    fn accepts_qualified_and_sized_types() {
        let diff = SchemaDiff {
            added_tables: vec![TableDiff {
                name: "t".to_string(),
                columns: vec![
                    col("a", "character varying(255)", true, None),
                    col("b", "numeric(10,2)", true, None),
                    col("c", "pg_catalog.text[]", true, None),
                ],
            }],
            removed_tables: vec![],
            modified_tables: vec![],
        };
        let sql = render_migration_sql(&diff).unwrap();
        assert!(sql.contains("character varying(255)"), "{sql}");
        assert!(sql.contains("numeric(10,2)"), "{sql}");
        assert!(sql.contains("pg_catalog.text[]"), "{sql}");
    }

    #[test]
    fn rejects_hostile_identifiers() {
        let diff = SchemaDiff {
            added_tables: vec![TableDiff {
                name: "t\"; DROP TABLE users; --".to_string(),
                columns: vec![col("c", "text", true, None)],
            }],
            removed_tables: vec![],
            modified_tables: vec![],
        };
        assert!(render_migration_sql(&diff).is_err());
    }

    #[test]
    fn renders_modified_columns() {
        let diff = SchemaDiff {
            added_tables: vec![],
            removed_tables: vec![],
            modified_tables: vec![TableModification {
                name: "orders".to_string(),
                added_columns: vec![col("note", "text", true, None)],
                removed_columns: vec![col("legacy", "text", true, None)],
                modified_columns: vec![ColumnChange {
                    name: "total".to_string(),
                    old_type: "integer".to_string(),
                    new_type: "numeric".to_string(),
                    old_nullable: false,
                    new_nullable: true,
                }],
            }],
        };
        let sql = render_migration_sql(&diff).unwrap();
        assert!(sql.contains("ADD COLUMN \"note\" text"), "{sql}");
        assert!(sql.contains("DROP COLUMN \"legacy\""), "{sql}");
        assert!(sql.contains("ALTER COLUMN \"total\" TYPE numeric"), "{sql}");
        assert!(sql.contains("DROP NOT NULL"), "{sql}");
    }

    #[test]
    fn renders_drop_table() {
        let diff = SchemaDiff {
            added_tables: vec![],
            removed_tables: vec![TableDiff {
                name: "old_logs".to_string(),
                columns: vec![],
            }],
            modified_tables: vec![],
        };
        let sql = render_migration_sql(&diff).unwrap();
        assert!(sql.contains("DROP TABLE \"old_logs\""), "{sql}");
    }

    #[test]
    fn empty_diff_reports_identical() {
        let diff = SchemaDiff {
            added_tables: vec![],
            removed_tables: vec![],
            modified_tables: vec![],
        };
        let sql = render_migration_sql(&diff).unwrap();
        assert!(sql.contains("identical"), "{sql}");
    }
}
