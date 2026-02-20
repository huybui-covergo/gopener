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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_default() {
        let settings = Settings::new();
        assert_eq!(settings.default_folder_id, None);
        assert_eq!(settings.default_folder_name, None);
        assert!(settings.auto_open_after_upload);
        assert!(!settings.auto_close_after_upload);
        assert_eq!(settings.theme, "dark");
        assert!(settings.recent_files.is_empty());
    }

    #[test]
    fn test_settings_derive_default() {
        let settings = Settings::default();
        assert_eq!(settings.theme, "");
        assert!(settings.recent_files.is_empty());
    }

    #[test]
    fn test_settings_serialization() {
        let settings = Settings::new();
        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("\"theme\":\"dark\""));
        assert!(json.contains("\"auto_open_after_upload\":true"));
        assert!(json.contains("\"auto_close_after_upload\":false"));
    }

    #[test]
    fn test_settings_deserialization() {
        let json = r#"{
            "default_folder_id": "folder-123",
            "default_folder_name": "My Folder",
            "auto_open_after_upload": false,
            "auto_close_after_upload": true,
            "theme": "light",
            "recent_files": []
        }"#;

        let settings: Settings = serde_json::from_str(json).unwrap();
        assert_eq!(settings.default_folder_id, Some("folder-123".to_string()));
        assert_eq!(settings.default_folder_name, Some("My Folder".to_string()));
        assert!(!settings.auto_open_after_upload);
        assert!(settings.auto_close_after_upload);
        assert_eq!(settings.theme, "light");
        assert!(settings.recent_files.is_empty());
    }

    #[test]
    fn test_settings_roundtrip() {
        let mut settings = Settings::new();
        settings.default_folder_id = Some("abc".to_string());
        settings.theme = "light".to_string();
        settings.recent_files.push(RecentFile {
            id: "file-1".to_string(),
            name: "test.docx".to_string(),
            path: "/path/to/test.docx".to_string(),
            google_url: "https://docs.google.com/document/d/abc".to_string(),
            file_type: "Google Docs".to_string(),
            uploaded_at: 1700000000,
        });

        let json = serde_json::to_string_pretty(&settings).unwrap();
        let deserialized: Settings = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.default_folder_id, settings.default_folder_id);
        assert_eq!(deserialized.theme, settings.theme);
        assert_eq!(deserialized.recent_files.len(), 1);
        assert_eq!(deserialized.recent_files[0].id, "file-1");
        assert_eq!(deserialized.recent_files[0].name, "test.docx");
    }

    #[test]
    fn test_recent_file_serialization() {
        let file = RecentFile {
            id: "rf-1".to_string(),
            name: "report.xlsx".to_string(),
            path: "/home/user/report.xlsx".to_string(),
            google_url: "https://docs.google.com/spreadsheets/d/xyz".to_string(),
            file_type: "Google Sheets".to_string(),
            uploaded_at: 1700000000,
        };

        let json = serde_json::to_string(&file).unwrap();
        let deserialized: RecentFile = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, "rf-1");
        assert_eq!(deserialized.name, "report.xlsx");
        assert_eq!(deserialized.uploaded_at, 1700000000);
    }

    #[test]
    fn test_recent_file_clone() {
        let file = RecentFile {
            id: "rf-1".to_string(),
            name: "test.docx".to_string(),
            path: "/test.docx".to_string(),
            google_url: "https://...".to_string(),
            file_type: "Google Docs".to_string(),
            uploaded_at: 1700000000,
        };

        let cloned = file.clone();
        assert_eq!(cloned.id, file.id);
        assert_eq!(cloned.name, file.name);
    }

    #[test]
    fn test_settings_with_multiple_recent_files() {
        let mut settings = Settings::new();
        for i in 0..15 {
            settings.recent_files.push(RecentFile {
                id: format!("file-{}", i),
                name: format!("test-{}.docx", i),
                path: format!("/path/test-{}.docx", i),
                google_url: format!("https://docs.google.com/document/d/{}", i),
                file_type: "Google Docs".to_string(),
                uploaded_at: 1700000000 + i,
            });
        }

        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: Settings = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.recent_files.len(), 15);
    }
}
