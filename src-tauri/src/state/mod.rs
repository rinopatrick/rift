use rusqlite::{Connection, Result as SqlResult};
use serde::Serialize;
use crate::db::connection::{ConnectionConfig, ConnectionInfo};

#[derive(Debug, Clone, Serialize)]
pub struct QueryHistoryItem {
    pub id: String,
    pub connection_id: String,
    pub query: String,
    pub executed_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct QueryBookmark {
    pub id: String,
    pub connection_id: String,
    pub name: String,
    pub query: String,
    pub created_at: String,
}

pub struct AppState {
    pub db: Connection,
}

impl AppState {
    pub fn new(app_dir: &std::path::Path) -> SqlResult<Self> {
        let db_path = app_dir.join("rift.db");
        let db = Connection::open(db_path)?;

        db.execute(
            "CREATE TABLE IF NOT EXISTS connections (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                driver TEXT NOT NULL DEFAULT 'postgres',
                host TEXT NOT NULL,
                port INTEGER NOT NULL,
                database TEXT NOT NULL,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                ssl_mode TEXT NOT NULL,
                file_path TEXT NOT NULL DEFAULT '',
                folder TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        // Migration: add folder column if it doesn't exist (ignore error)
        let _ = db.execute("ALTER TABLE connections ADD COLUMN folder TEXT NOT NULL DEFAULT ''", []);

        db.execute(
            "CREATE TABLE IF NOT EXISTS query_history (
                id TEXT PRIMARY KEY,
                connection_id TEXT NOT NULL,
                query TEXT NOT NULL,
                executed_at TEXT NOT NULL
            )",
            [],
        )?;

        db.execute(
            "CREATE TABLE IF NOT EXISTS query_bookmarks (
                id TEXT PRIMARY KEY,
                connection_id TEXT NOT NULL,
                name TEXT NOT NULL,
                query TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        db.execute(
            "CREATE TABLE IF NOT EXISTS app_settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Self { db })
    }

    pub fn save_connection(&self, config: &ConnectionConfig) -> SqlResult<()> {
        self.db.execute(
            "INSERT OR REPLACE INTO connections (id, name, driver, host, port, database, username, password, ssl_mode, file_path, folder, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            [
                &config.id, &config.name, &config.driver, &config.host, &config.port.to_string(),
                &config.database, &config.username, &config.password, &config.ssl_mode, &config.file_path, &config.folder, &config.created_at,
            ],
        )?;
        Ok(())
    }

    pub fn get_connections(&self) -> SqlResult<Vec<ConnectionInfo>> {
        let mut stmt = self.db.prepare(
            "SELECT id, name, driver, host, port, database, username, ssl_mode, file_path, folder, created_at FROM connections ORDER BY folder, created_at DESC"
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(ConnectionInfo {
                id: row.get(0)?,
                name: row.get(1)?,
                driver: row.get(2)?,
                host: row.get(3)?,
                port: row.get(4)?,
                database: row.get(5)?,
                username: row.get(6)?,
                ssl_mode: row.get(7)?,
                file_path: row.get(8)?,
                folder: row.get(9)?,
                created_at: row.get(10)?,
            })
        })?;

        rows.collect()
    }

    pub fn delete_connection(&self, id: &str) -> SqlResult<()> {
        self.db.execute("DELETE FROM connections WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn get_connection_config(&self, id: &str) -> SqlResult<Option<ConnectionConfig>> {
        let mut stmt = self.db.prepare(
            "SELECT id, name, driver, host, port, database, username, password, ssl_mode, file_path, folder, created_at FROM connections WHERE id = ?1"
        )?;

        let mut rows = stmt.query_map([id], |row| {
            Ok(ConnectionConfig {
                id: row.get(0)?,
                name: row.get(1)?,
                driver: row.get(2)?,
                host: row.get(3)?,
                port: row.get(4)?,
                database: row.get(5)?,
                username: row.get(6)?,
                password: row.get(7)?,
                ssl_mode: row.get(8)?,
                file_path: row.get(9)?,
                folder: row.get(10)?,
                created_at: row.get(11)?,
            })
        })?;

        rows.next().transpose()
    }
    pub fn save_query_history(&self, connection_id: &str, query: &str) -> SqlResult<()> {
        let id = uuid::Uuid::new_v4().to_string();
        let executed_at = time::OffsetDateTime::now_utc().to_string();
        self.db.execute(
            "INSERT INTO query_history (id, connection_id, query, executed_at) VALUES (?1, ?2, ?3, ?4)",
            [&id, connection_id, query, &executed_at],
        )?;
        Ok(())
    }

    pub fn get_query_history(&self, connection_id: &str) -> SqlResult<Vec<QueryHistoryItem>> {
        let mut stmt = self.db.prepare(
            "SELECT id, connection_id, query, executed_at FROM query_history WHERE connection_id = ?1 ORDER BY executed_at DESC LIMIT 100"
        )?;

        let rows = stmt.query_map([connection_id], |row| {
            Ok(QueryHistoryItem {
                id: row.get(0)?,
                connection_id: row.get(1)?,
                query: row.get(2)?,
                executed_at: row.get(3)?,
            })
        })?;

        rows.collect()
    }

    pub fn save_bookmark(&self, connection_id: &str, name: &str, query: &str) -> SqlResult<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = time::OffsetDateTime::now_utc().to_string();
        self.db.execute(
            "INSERT INTO query_bookmarks (id, connection_id, name, query, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            [&id, connection_id, name, query, &created_at],
        )?;
        Ok(id)
    }

    pub fn get_bookmarks(&self, connection_id: &str) -> SqlResult<Vec<QueryBookmark>> {
        let mut stmt = self.db.prepare(
            "SELECT id, connection_id, name, query, created_at FROM query_bookmarks WHERE connection_id = ?1 ORDER BY created_at DESC"
        )?;
        let rows = stmt.query_map([connection_id], |row| {
            Ok(QueryBookmark {
                id: row.get(0)?,
                connection_id: row.get(1)?,
                name: row.get(2)?,
                query: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;
        rows.collect()
    }

    pub fn delete_bookmark(&self, id: &str) -> SqlResult<()> {
        self.db.execute("DELETE FROM query_bookmarks WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn save_setting(&self, key: &str, value: &str) -> SqlResult<()> {
        self.db.execute(
            "INSERT OR REPLACE INTO app_settings (key, value) VALUES (?1, ?2)",
            [key, value],
        )?;
        Ok(())
    }

    pub fn get_setting(&self, key: &str) -> SqlResult<Option<String>> {
        let mut stmt = self.db.prepare("SELECT value FROM app_settings WHERE key = ?1")?;
        let mut rows = stmt.query_map([key], |row| row.get(0))?;
        rows.next().transpose()
    }

    pub fn get_all_settings(&self) -> SqlResult<Vec<(String, String)>> {
        let mut stmt = self.db.prepare("SELECT key, value FROM app_settings")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;
        rows.collect()
    }
}
