pub fn quote_ident(name: &str) -> Result<String, String> {
    if name.is_empty() || name.len() > 128 {
        return Err(format!("Invalid identifier: {name}"));
    }
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '$')
    {
        return Err(format!("Invalid identifier: {name}"));
    }
    Ok(format!("\"{}\"", name.replace('"', "\"\"")))
}

pub fn quote_mysql_ident(name: &str) -> Result<String, String> {
    if name.is_empty() || name.len() > 64 {
        return Err(format!("Invalid identifier: {name}"));
    }
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '$')
    {
        return Err(format!("Invalid identifier: {name}"));
    }
    Ok(format!("`{}`", name.replace('`', "``")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_and_injection() {
        assert!(quote_ident("").is_err());
        assert!(quote_ident("a\"; DROP TABLE x; --").is_err());
        assert!(quote_ident("a-b").is_err());
        assert!(quote_ident("a b").is_err());
        assert_eq!(quote_ident("users").unwrap(), "\"users\"");
    }

    #[test]
    fn rejects_oversized_identifier() {
        let long = "a".repeat(129);
        assert!(quote_ident(&long).is_err());
    }

    #[test]
    fn mysql_rejects_backtick_injection() {
        assert!(quote_mysql_ident("a`b").is_err());
        assert!(quote_mysql_ident("a-b").is_err());
        assert_eq!(quote_mysql_ident("users").unwrap(), "`users`");
    }

    #[test]
    fn mysql_rejects_backslash_escape_injection() {
        assert!(quote_mysql_ident("a\\").is_err());
        assert!(quote_mysql_ident("db\\'; DROP TABLE x; --").is_err());
    }

    #[test]
    fn mysql_allows_dollar_and_underscore() {
        assert_eq!(quote_mysql_ident("my$table_1").unwrap(), "`my$table_1`");
    }
}
