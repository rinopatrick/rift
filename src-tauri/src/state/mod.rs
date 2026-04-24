use rusqlite::{Connection, Result as SqlResult};
use crate::db::connection::{ConnectionConfig, ConnectionInfo};

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
                host TEXT NOT NULL,
                port INTEGER NOT NULL,
                database TEXT NOT NULL,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                ssl_mode TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;
        
        db.execute(
            "CREATE TABLE IF NOT EXISTS query_history (
                id TEXT PRIMARY KEY,
                connection_id TEXT NOT NULL,
                query TEXT NOT NULL,
                executed_at TEXT NOT NULL
            )",
            [],
        )?;
        
        Ok(Self { db })
    }
    
    pub fn save_connection(&self, config: &ConnectionConfig) -> SqlResult<()> {
        self.db.execute(
            "INSERT OR REPLACE INTO connections (id, name, host, port, database, username, password, ssl_mode, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            [
                &config.id, &config.name, &config.host, &config.port.to_string(),
                &config.database, &config.username, &config.password, &config.ssl_mode, &config.created_at,
            ],
        )?;
        Ok(())
    }
    
    pub fn get_connections(&self) -> SqlResult<Vec<ConnectionInfo>> {
        let mut stmt = self.db.prepare(
            "SELECT id, name, host, port, database, username, ssl_mode, created_at FROM connections ORDER BY created_at DESC"
        )?;
        
        let rows = stmt.query_map([], |row| {
            Ok(ConnectionInfo {
                id: row.get(0)?,
                name: row.get(1)?,
                host: row.get(2)?,
                port: row.get(3)?,
                database: row.get(4)?,
                username: row.get(5)?,
                ssl_mode: row.get(6)?,
                created_at: row.get(7)?,
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
            "SELECT id, name, host, port, database, username, password, ssl_mode, created_at FROM connections WHERE id = ?1"
        )?;
        
        let mut rows = stmt.query_map([id], |row| {
            Ok(ConnectionConfig {
                id: row.get(0)?,
                name: row.get(1)?,
                host: row.get(2)?,
                port: row.get(3)?,
                database: row.get(4)?,
                username: row.get(5)?,
                password: row.get(6)?,
                ssl_mode: row.get(7)?,
                created_at: row.get(8)?,
            })
        })?;
        
        rows.next().transpose()
    }
}
