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
    let path = plugins_dir(app_dir).join(filename);
    std::fs::read(&path).map_err(|e| e.to_string())
}
