/// Translate low-level database errors into human-readable messages.
pub fn humanize_error(err: &str) -> String {
    let lower = err.to_lowercase();

    // Connection / network errors
    if lower.contains("timeout") || lower.contains("timed out") {
        return "Connection timed out. Check that the host is reachable and the port is correct."
            .to_string();
    }
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
        return "SSL/TLS connection failed. Try changing the SSL mode in connection settings (e.g. prefer, require, disable).".to_string();
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
