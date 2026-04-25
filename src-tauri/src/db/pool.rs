use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;
use crate::db::connection::ConnectionConfig;

#[derive(Debug, thiserror::Error)]
pub enum PoolError {
    #[error("pool creation failed: {0}")]
    Create(#[from] deadpool_postgres::CreatePoolError),
    #[error("postgres error: {0}")]
    Postgres(#[from] tokio_postgres::Error),
    #[error("mysql error: {0}")]
    Mysql(#[from] mysql_async::Error),
    #[error("pool exhausted")]
    Exhausted,
}

pub fn create_pool(config: &ConnectionConfig) -> Result<Pool, PoolError> {
    let mut cfg = Config::new();
    cfg.host = Some(config.host.clone());
    cfg.port = Some(config.port);
    cfg.dbname = Some(config.database.clone());
    cfg.user = Some(config.username.clone());
    cfg.password = Some(config.password.clone());
    cfg.manager = Some(deadpool_postgres::ManagerConfig {
        recycling_method: deadpool_postgres::RecyclingMethod::Fast,
    });

    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
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
