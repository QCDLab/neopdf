//! Persist and apply GUI configuration (grid folder, Python binary, etc.).
//! Settings are stored in a JSON file and applied to the process environment
//! at startup.

use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

const CONFIG_FILENAME: &str = "neopdf_gui_config.json";

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GuiConfig {
    /// Folder from which PDF grids are loaded (equivalent to NEOPDF_DATA_PATH).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_path: Option<String>,
    /// Path to the Python binary used for matplotlib plotting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python_path: Option<String>,
}

fn config_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir: PathBuf = app.path().app_data_dir().map_err(|e| e.to_string())?;
    Ok(dir.join(CONFIG_FILENAME))
}

/// Load config from app data dir. Returns default config if file missing or invalid.
pub fn load_config(app: &AppHandle) -> Result<GuiConfig, String> {
    let path = config_path(app)?;
    if !path.exists() {
        return Ok(GuiConfig::default());
    }
    let s = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&s).map_err(|e| e.to_string())
}

/// Save config to app data dir. Creates parent dirs if needed.
pub fn save_config(app: &AppHandle, config: &GuiConfig) -> Result<(), String> {
    let path = config_path(app)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let s = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(&path, s).map_err(|e| e.to_string())
}

/// Apply stored config to the process environment. Call once at startup.
///
/// - `NEOPDF_DATA_PATH` — grid data folder for the neopdf library.
/// - `NEOPDF_PYTHON`    — Python binary used for matplotlib plotting.
pub fn apply_config_to_env(app: &AppHandle) -> Result<(), String> {
    let config = load_config(app)?;
    if let Some(ref path) = config.data_path {
        let trimmed = path.trim();
        if !trimmed.is_empty() {
            std::env::set_var("NEOPDF_DATA_PATH", trimmed);
        }
    }
    if let Some(ref path) = config.python_path {
        let trimmed = path.trim();
        if !trimmed.is_empty() {
            std::env::set_var("NEOPDF_PYTHON", trimmed);
        }
    }
    Ok(())
}
