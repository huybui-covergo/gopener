/// Register file associations for the application
/// Note: Most file associations are handled via tauri.conf.json bundle settings
/// This command provides runtime registration for platforms that support it

#[tauri::command]
pub async fn register_file_associations() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        register_windows_associations()
    }

    #[cfg(target_os = "macos")]
    {
        // macOS handles file associations via Info.plist in the bundle
        Ok("File associations are configured in the app bundle".to_string())
    }

    #[cfg(target_os = "linux")]
    {
        register_linux_associations()
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err("Unsupported platform".to_string())
    }
}

#[cfg(target_os = "windows")]
fn register_windows_associations() -> Result<String, String> {
    // On Windows, file associations are typically set during installation
    // via the installer (NSIS/WiX) or can be set via registry
    // For now, we rely on the Tauri bundle configuration
    Ok("File associations should be configured during installation".to_string())
}

#[cfg(target_os = "linux")]
fn register_linux_associations() -> Result<String, String> {
    use std::fs;
    use std::path::PathBuf;

    // Create .desktop file for Linux
    let desktop_entry = r#"[Desktop Entry]
Name=Gopener
Comment=Upload Office files to Google Drive
Exec=gopener %F
Icon=gopener
Type=Application
Categories=Office;Utility;
MimeType=application/msword;application/vnd.openxmlformats-officedocument.wordprocessingml.document;application/vnd.oasis.opendocument.text;application/rtf;text/plain;application/vnd.ms-excel;application/vnd.openxmlformats-officedocument.spreadsheetml.sheet;application/vnd.oasis.opendocument.spreadsheet;text/csv;text/tab-separated-values;application/vnd.ms-powerpoint;application/vnd.openxmlformats-officedocument.presentationml.presentation;application/vnd.oasis.opendocument.presentation;
"#;

    // Get applications directory
    let home = std::env::var("HOME").map_err(|_| "HOME not set")?;
    let apps_dir = PathBuf::from(&home).join(".local/share/applications");

    // Create directory if needed
    if !apps_dir.exists() {
        fs::create_dir_all(&apps_dir).map_err(|e| format!("Failed to create apps dir: {}", e))?;
    }

    // Write desktop file
    let desktop_path = apps_dir.join("gopener.desktop");
    fs::write(&desktop_path, desktop_entry)
        .map_err(|e| format!("Failed to write desktop file: {}", e))?;

    // Update desktop database
    let _ = std::process::Command::new("update-desktop-database")
        .arg(&apps_dir)
        .output();

    Ok(format!(
        "Desktop file created at {}",
        desktop_path.display()
    ))
}
