use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub id: String,
    pub name: String,
    pub driver: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub ssl_mode: String,
    pub file_path: String,
    pub created_at: String,
}

impl ConnectionConfig {
    pub fn new(name: String, host: String, port: u16, database: String, username: String, password: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            driver: "postgres".to_string(),
            host,
            port,
            database,
            username,
            password,
            ssl_mode: "prefer".to_string(),
            file_path: "".to_string(),
            created_at: time::OffsetDateTime::now_utc().to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ConnectionInfo {
    pub id: String,
    pub name: String,
    pub driver: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub ssl_mode: String,
    pub file_path: String,
    pub created_at: String,
}

impl From<ConnectionConfig> for ConnectionInfo {
    fn from(c: ConnectionConfig) -> Self {
        Self {
            id: c.id,
            name: c.name,
            driver: c.driver,
            host: c.host,
            port: c.port,
            database: c.database,
            username: c.username,
            ssl_mode: c.ssl_mode,
            file_path: c.file_path,
            created_at: c.created_at,
        }
    }
}
