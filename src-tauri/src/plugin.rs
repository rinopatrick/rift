use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize)]
pub struct PluginInfo {
    pub name: String,
    pub filename: String,
}

pub fn plugins_dir(app_dir: &std::path::Path) -> PathBuf {
    app_dir.join("plugins")
}

pub fn list_plugins(app_dir: &std::path::Path) -> Result<Vec<PluginInfo>, String> {
    let dir = plugins_dir(app_dir);
    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut plugins = Vec::new();
    for entry in std::fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("wasm") {
            let filename = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            // Use filename without extension as name
            let name = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
            plugins.push(PluginInfo { name, filename });
        }
    }

    Ok(plugins)
}


pub fn read_plugin(app_dir: &std::path::Path, filename: &str) -> Result<Vec<u8>, String> {
    if filename.is_empty() || filename.len() > 128 {
        return Err("Invalid plugin filename".to_string());
    }
    if !filename
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.')
    {
        return Err("Invalid plugin filename".to_string());
    }
    if !filename.ends_with(".wasm") || filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return Err("Invalid plugin filename".to_string());
    }
    let dir = plugins_dir(app_dir);
    let path = dir.join(filename);
    if path.parent() != Some(dir.as_path()) {
        return Err("Invalid plugin filename".to_string());
    }
    std::fs::read(&path).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn rejects_traversal_and_separators() {
        let dir = std::env::temp_dir();
        assert!(super::read_plugin(&dir, "../evil.wasm").is_err());
        assert!(super::read_plugin(&dir, "a/b.wasm").is_err());
        assert!(super::read_plugin(&dir, "a\\b.wasm").is_err());
        assert!(super::read_plugin(&dir, "plugin.js").is_err());
        assert!(super::read_plugin(&dir, "").is_err());
    }
}
