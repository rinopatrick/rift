use crate::db::connection::ConnectionConfig;
use deadpool_postgres::{Config, Pool, Runtime};

#[derive(Debug, thiserror::Error)]
pub enum PoolError {
    #[error("pool creation failed: {0}")]
    Create(#[from] deadpool_postgres::CreatePoolError),
    #[error("postgres error: {0}")]
    Postgres(#[from] tokio_postgres::Error),
    #[error("mysql error: {0}")]
    Mysql(#[from] mysql_async::Error),
    #[error("TLS setup failed: {0}")]
    Tls(String),
}

/// Resolved TLS policy for a connection. Pure function so the mapping
/// from user-facing `ssl_mode` strings to deadpool behavior is unit-testable
/// without opening any socket.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsPolicy {
    Disabled,
    Prefer,
    Require,
}

pub fn resolve_ssl_policy(raw: &str) -> TlsPolicy {
    match raw.trim().to_lowercase().as_str() {
        "disable" => TlsPolicy::Disabled,
        "require" => TlsPolicy::Require,
        _ => TlsPolicy::Prefer,
    }
}

fn deadpool_ssl_mode(policy: TlsPolicy) -> deadpool_postgres::SslMode {
    match policy {
        TlsPolicy::Disabled => deadpool_postgres::SslMode::Disable,
        TlsPolicy::Require => deadpool_postgres::SslMode::Require,
        TlsPolicy::Prefer => deadpool_postgres::SslMode::Prefer,
    }
}

/// Timeout applied while establishing a new pooled connection.
pub const CONNECT_TIMEOUT_SECS: u64 = 10;

pub fn create_pool(config: &ConnectionConfig) -> Result<Pool, PoolError> {
    let policy = resolve_ssl_policy(&config.ssl_mode);
    let mut cfg = Config::new();
    cfg.host = Some(config.host.clone());
    cfg.port = Some(config.port);
    cfg.dbname = Some(config.database.clone());
    cfg.user = Some(config.username.clone());
    cfg.password = Some(config.password.clone());
    cfg.ssl_mode = Some(deadpool_ssl_mode(policy));
    cfg.connect_timeout = Some(std::time::Duration::from_secs(CONNECT_TIMEOUT_SECS));
    cfg.manager = Some(deadpool_postgres::ManagerConfig {
        recycling_method: deadpool_postgres::RecyclingMethod::Fast,
    });

    if policy == TlsPolicy::Disabled {
        let pool = cfg.create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)?;
        return Ok(pool);
    }

    let connector = native_tls::TlsConnector::builder()
        .build()
        .map_err(|e| PoolError::Tls(e.to_string()))?;
    let tls = postgres_native_tls::MakeTlsConnector::new(connector);
    let pool = cfg.create_pool(Some(Runtime::Tokio1), tls)?;
    Ok(pool)
}

pub fn create_mysql_pool(config: &ConnectionConfig) -> Result<mysql_async::Pool, PoolError> {
    let opts: mysql_async::Opts = mysql_async::OptsBuilder::default()
        .ip_or_hostname(&config.host)
        .tcp_port(config.port)
        .db_name(Some(&config.database))
        .user(Some(&config.username))
        .pass(Some(&config.password))
        .into();
    Ok(mysql_async::Pool::new(opts))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_disable_case_insensitively() {
        assert_eq!(resolve_ssl_policy("disable"), TlsPolicy::Disabled);
        assert_eq!(resolve_ssl_policy("DISABLE"), TlsPolicy::Disabled);
        assert_eq!(resolve_ssl_policy("  disable  "), TlsPolicy::Disabled);
    }

    #[test]
    fn resolves_require() {
        assert_eq!(resolve_ssl_policy("require"), TlsPolicy::Require);
    }

    #[test]
    fn unknown_modes_fall_back_to_prefer() {
        assert_eq!(resolve_ssl_policy("prefer"), TlsPolicy::Prefer);
        assert_eq!(resolve_ssl_policy(""), TlsPolicy::Prefer);
        assert_eq!(resolve_ssl_policy("bogus-mode"), TlsPolicy::Prefer);
    }
}
