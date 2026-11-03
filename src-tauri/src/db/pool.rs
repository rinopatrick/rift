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

fn ssl_mode(config: &ConnectionConfig) -> deadpool_postgres::SslMode {
    match config.ssl_mode.to_lowercase().as_str() {
        "disable" => deadpool_postgres::SslMode::Disable,
        "require" => deadpool_postgres::SslMode::Require,
        _ => deadpool_postgres::SslMode::Prefer,
    }
}

pub fn create_pool(config: &ConnectionConfig) -> Result<Pool, PoolError> {
    let mut cfg = Config::new();
    cfg.host = Some(config.host.clone());
    cfg.port = Some(config.port);
    cfg.dbname = Some(config.database.clone());
    cfg.user = Some(config.username.clone());
    cfg.password = Some(config.password.clone());
    cfg.ssl_mode = Some(ssl_mode(config));
    cfg.connect_timeout = Some(std::time::Duration::from_secs(10));
    cfg.manager = Some(deadpool_postgres::ManagerConfig {
        recycling_method: deadpool_postgres::RecyclingMethod::Fast,
    });

    if ssl_mode(config) == deadpool_postgres::SslMode::Disable {
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
