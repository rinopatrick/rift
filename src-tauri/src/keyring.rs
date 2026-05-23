const SERVICE: &str = "dev.rift.app";

fn key_for_connection(id: &str, field: &str) -> String {
    format!("rift_conn_{}_{}", id, field)
}

pub fn set_password(id: &str, password: &str) -> Result<(), String> {
    if password.is_empty() {
        delete_password(id)?;
        return Ok(());
    }
    let entry = keyring::Entry::new(SERVICE, &key_for_connection(id, "password"))
        .map_err(|e| e.to_string())?;
    entry.set_password(password).map_err(|e| e.to_string())
}

pub fn get_password(id: &str) -> Option<String> {
    let entry = keyring::Entry::new(SERVICE, &key_for_connection(id, "password")).ok()?;
    entry.get_password().ok()
}

pub fn delete_password(id: &str) -> Result<(), String> {
    let entry = keyring::Entry::new(SERVICE, &key_for_connection(id, "password"))
        .map_err(|e| e.to_string())?;
    match entry.delete_credential() {
        Ok(_) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn set_ssh_password(id: &str, password: &str) -> Result<(), String> {
    if password.is_empty() {
        delete_ssh_password(id)?;
        return Ok(());
    }
    let entry = keyring::Entry::new(SERVICE, &key_for_connection(id, "ssh_password"))
        .map_err(|e| e.to_string())?;
    entry.set_password(password).map_err(|e| e.to_string())
}

pub fn get_ssh_password(id: &str) -> Option<String> {
    let entry = keyring::Entry::new(SERVICE, &key_for_connection(id, "ssh_password")).ok()?;
    entry.get_password().ok()
}

pub fn delete_ssh_password(id: &str) -> Result<(), String> {
    let entry = keyring::Entry::new(SERVICE, &key_for_connection(id, "ssh_password"))
        .map_err(|e| e.to_string())?;
    match entry.delete_credential() {
        Ok(_) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn set_ssh_passphrase(id: &str, passphrase: &str) -> Result<(), String> {
    if passphrase.is_empty() {
        delete_ssh_passphrase(id)?;
        return Ok(());
    }
    let entry = keyring::Entry::new(SERVICE, &key_for_connection(id, "ssh_passphrase"))
        .map_err(|e| e.to_string())?;
    entry.set_password(passphrase).map_err(|e| e.to_string())
}

pub fn get_ssh_passphrase(id: &str) -> Option<String> {
    let entry = keyring::Entry::new(SERVICE, &key_for_connection(id, "ssh_passphrase")).ok()?;
    entry.get_password().ok()
}

pub fn delete_ssh_passphrase(id: &str) -> Result<(), String> {
    let entry = keyring::Entry::new(SERVICE, &key_for_connection(id, "ssh_passphrase"))
        .map_err(|e| e.to_string())?;
    match entry.delete_credential() {
        Ok(_) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
