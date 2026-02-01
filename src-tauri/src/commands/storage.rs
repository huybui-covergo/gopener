use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecentFile {
    pub id: String,
    pub name: String,
    pub path: String,
    pub google_url: String,
    pub file_type: String,
    pub uploaded_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Settings {
    pub default_folder_id: Option<String>,
    pub default_folder_name: Option<String>,
    pub auto_open_after_upload: bool,
    pub auto_close_after_upload: bool,
    pub theme: String,
    pub recent_files: Vec<RecentFile>,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            default_folder_id: None,
            default_folder_name: None,
            auto_open_after_upload: true,
            auto_close_after_upload: false,
            theme: "dark".to_string(),
            recent_files: Vec::new(),
        }
    }
}

/// Get the settings file path
fn get_settings_path() -> Result<PathBuf, String> {
    let config_dir = dirs::config_dir().ok_or("Could not find config directory")?;
    let app_dir = config_dir.join("gopener");

    // Create directory if it doesn't exist
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).map_err(|e| format!("Failed to create config dir: {}", e))?;
    }

    Ok(app_dir.join("settings.json"))
}

/// Load settings from disk
#[tauri::command]
pub async fn get_settings() -> Result<Settings, String> {
    let path = get_settings_path()?;

    if !path.exists() {
        return Ok(Settings::new());
    }

    let contents =
        fs::read_to_string(&path).map_err(|e| format!("Failed to read settings: {}", e))?;

    serde_json::from_str(&contents).map_err(|e| format!("Failed to parse settings: {}", e))
}

/// Save settings to disk
#[tauri::command]
pub async fn save_settings(settings: Settings) -> Result<(), String> {
    let path = get_settings_path()?;

    let contents =
        serde_json::to_string_pretty(&settings).map_err(|e| format!("Failed to serialize: {}", e))?;

    fs::write(&path, contents).map_err(|e| format!("Failed to write settings: {}", e))?;

    Ok(())
}

/// Add a file to recent files history
#[tauri::command]
pub async fn add_recent_file(file: RecentFile) -> Result<(), String> {
    let mut settings = get_settings().await?;

    // Remove if already exists (to move to top)
    settings.recent_files.retain(|f| f.id != file.id);

    // Add to front
    settings.recent_files.insert(0, file);

    // Keep only last 10 files
    settings.recent_files.truncate(10);

    save_settings(settings).await
}

/// Clear all recent files
#[tauri::command]
pub async fn clear_recent_files() -> Result<(), String> {
    let mut settings = get_settings().await?;
    settings.recent_files.clear();
    save_settings(settings).await
}
