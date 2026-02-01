use crate::utils::keychain::{self, keys};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OAuthConfig {
    pub use_custom: bool,
    pub client_id: Option<String>,
    pub has_client_secret: bool,
}

/// Get the current OAuth configuration
#[tauri::command]
pub async fn get_oauth_config() -> Result<OAuthConfig, String> {
    let client_id = keychain::retrieve(keys::CUSTOM_CLIENT_ID).map_err(|e| e.to_string())?;

    let has_secret = keychain::retrieve(keys::CUSTOM_CLIENT_SECRET)
        .map_err(|e| e.to_string())?
        .map(|s| !s.is_empty())
        .unwrap_or(false);

    let use_custom = client_id.as_ref().map(|s| !s.is_empty()).unwrap_or(false);

    Ok(OAuthConfig {
        use_custom,
        client_id,
        has_client_secret: has_secret,
    })
}

/// Save custom OAuth credentials
#[tauri::command]
pub async fn save_oauth_config(client_id: String, client_secret: String) -> Result<(), String> {
    // Validate client ID format (basic validation)
    if !client_id.contains(".apps.googleusercontent.com") {
        return Err("Invalid client ID format. It should end with .apps.googleusercontent.com".to_string());
    }

    keychain::store(keys::CUSTOM_CLIENT_ID, &client_id).map_err(|e| e.to_string())?;
    keychain::store(keys::CUSTOM_CLIENT_SECRET, &client_secret).map_err(|e| e.to_string())?;

    Ok(())
}

/// Clear custom OAuth credentials and revert to default
#[tauri::command]
pub async fn clear_oauth_config() -> Result<(), String> {
    let _ = keychain::delete(keys::CUSTOM_CLIENT_ID);
    let _ = keychain::delete(keys::CUSTOM_CLIENT_SECRET);
    Ok(())
}
