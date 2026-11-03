/// Translate low-level database errors into human-readable messages.
///
/// Timeout errors carry a `timeout_hint` so the UI can tell the user *which*
/// timeout fired: connection-establishment vs statement execution. Callers
/// that know the context (pool connect vs `SET statement_timeout`) should use
/// [`humanize_error_with_hint`]; the plain [`humanize_error`] keeps the old
/// generic wording.
pub fn humanize_error(err: &str) -> String {
    humanize_error_with_hint(err, None)
}

/// Extra context about which timeout applies, when known by the caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeoutHint {
    /// Failed while opening a connection (pool `connect_timeout`).
    Connect,
    /// A running statement exceeded `statement_timeout`.
    Statement { timeout_secs: u64 },
}

pub fn humanize_error_with_hint(err: &str, timeout_hint: Option<TimeoutHint>) -> String {
    let lower = err.to_lowercase();

    // Timeouts first: distinguish connect vs statement when the caller knows.
    if lower.contains("timeout")
        || lower.contains("timed out")
        || lower.contains("statement timeout")
        || lower.contains("canceling statement due to")
        || lower.contains("cancelling statement due to")
        || lower.contains("query timed out")
        || lower.contains("deadline exceeded")
    {
        match timeout_hint {
            Some(TimeoutHint::Statement { timeout_secs }) => {
                return format!(
                    "Query exceeded the {timeout_secs}s statement timeout. Narrow the query (add LIMIT / filters) or raise the timeout in Settings."
                );
            }
            Some(TimeoutHint::Connect) => {
                return format!(
                    "Connection timed out after {}s while reaching the server. Check that the host is reachable and the port is correct.",
                    crate::db::pool::CONNECT_TIMEOUT_SECS
                );
            }
            None => {
                return "Timed out. If connecting, check host/port reachability; if a query was running, narrow it (LIMIT / filters) or raise the query timeout in Settings.".to_string();
            }
        }
    }
    // MySQL connection loss surfaces without the word "timeout".
    if lower.contains("gone away") || lower.contains("connection reset") {
        return "Connection lost mid-query (server closed it or the network dropped). Check server load, wait_timeout settings, and network stability, then reconnect and retry.".to_string();
    }

    // Connection / network errors
    if lower.contains("connection refused") || lower.contains("os error 111") {
        return "Connection refused. The database server may not be running or the port may be incorrect.".to_string();
    }
    if lower.contains("no route to host")
        || lower.contains("unreachable")
        || lower.contains("network is unreachable")
    {
        return "Cannot reach the database host. Check your network and the host address."
            .to_string();
    }
    if lower.contains("name or service not known")
        || lower.contains("nodename nor servname provided")
        || lower.contains("getaddrinfo")
    {
        return "Host not found. Check that the hostname is spelled correctly.".to_string();
    }

    // SSL / TLS errors
    if lower.contains("ssl")
        || lower.contains("tls")
        || lower.contains("certificate")
        || lower.contains("handshake")
    {
        return "SSL/TLS connection failed. Open the connection settings and try a different SSL mode: use `disable` for local/plain servers, `prefer` for most cloud hosts, `require` when the server mandates encryption.".to_string();
    }

    // Authentication
    if lower.contains("authentication")
        || lower.contains("password authentication")
        || lower.contains("access denied")
    {
        return "Authentication failed. Check your username and password.".to_string();
    }
    if lower.contains("role") && lower.contains("does not exist") {
        return "User (role) does not exist. Check your username.".to_string();
    }

    // Database / schema
    if lower.contains("database") && lower.contains("does not exist") {
        return "Database not found. Check the database name.".to_string();
    }
    if lower.contains("catalog") && lower.contains("not found") {
        return "Database catalog not found. Check the database name.".to_string();
    }

    // Table / column
    if lower.contains("relation") && lower.contains("does not exist") {
        return "Table or view does not exist. Check your table name.".to_string();
    }
    if lower.contains("table")
        && (lower.contains("doesn't exist") || lower.contains("does not exist"))
    {
        return "Table does not exist. Check your table name.".to_string();
    }
    if lower.contains("column") && lower.contains("does not exist") {
        return "Column does not exist. Check your column name.".to_string();
    }

    // Permissions
    if lower.contains("permission denied") {
        return "Permission denied. Your user account may not have the required privileges."
            .to_string();
    }

    // Syntax
    if lower.contains("syntax error") {
        return format!("SQL syntax error. {}", err);
    }

    // Pool / resource
    if lower.contains("pool") && lower.contains("exhausted") {
        return "Connection pool exhausted. Too many concurrent queries.".to_string();
    }

    // Cancelled
    if lower.contains("cancelled") || lower.contains("canceled") {
        return "Query was cancelled.".to_string();
    }

    // SSH tunnel
    if lower.contains("ssh") && (lower.contains("connection") || lower.contains("refused")) {
        return "SSH tunnel connection failed. Check the SSH host, port, and credentials."
            .to_string();
    }

    // Fallback: return original with a prefix
    format!("Database error: {}", err)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statement_timeout_names_limit_and_settings() {
        let msg = humanize_error_with_hint(
            "canceling statement due to statement timeout",
            Some(TimeoutHint::Statement { timeout_secs: 30 }),
        );
        assert!(msg.contains("30s"), "{msg}");
        assert!(msg.contains("LIMIT"), "{msg}");
    }

    #[test]
    fn connect_timeout_names_host_and_port() {
        let msg = humanize_error_with_hint("connection timed out", Some(TimeoutHint::Connect));
        assert!(msg.contains("host"), "{msg}");
        assert!(msg.contains("port"), "{msg}");
    }

    #[test]
    fn mysql_gone_away_suggests_reconnect() {
        let msg = humanize_error("MySQL server has gone away");
        assert!(msg.contains("reconnect"), "{msg}");
    }

    #[test]
    fn ssl_error_names_all_three_modes() {
        let msg = humanize_error("tls handshake failure: certificate verify failed");
        assert!(msg.contains("disable"), "{msg}");
        assert!(msg.contains("prefer"), "{msg}");
        assert!(msg.contains("require"), "{msg}");
    }

    #[test]
    fn plain_timeout_without_hint_covers_both_causes() {
        let msg = humanize_error("operation timed out");
        assert!(msg.contains("host/port"), "{msg}");
        assert!(msg.contains("LIMIT"), "{msg}");
    }
}
