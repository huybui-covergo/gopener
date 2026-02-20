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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth_config_serialization() {
        let config = OAuthConfig {
            use_custom: true,
            client_id: Some("test-id.apps.googleusercontent.com".to_string()),
            has_client_secret: true,
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"use_custom\":true"));
        assert!(json.contains("\"has_client_secret\":true"));
    }

    #[test]
    fn test_oauth_config_deserialization() {
        let json = r#"{
            "use_custom": false,
            "client_id": null,
            "has_client_secret": false
        }"#;

        let config: OAuthConfig = serde_json::from_str(json).unwrap();
        assert!(!config.use_custom);
        assert!(config.client_id.is_none());
        assert!(!config.has_client_secret);
    }

    #[test]
    fn test_oauth_config_roundtrip() {
        let config = OAuthConfig {
            use_custom: true,
            client_id: Some("my-id.apps.googleusercontent.com".to_string()),
            has_client_secret: false,
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: OAuthConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.use_custom, config.use_custom);
        assert_eq!(deserialized.client_id, config.client_id);
        assert_eq!(deserialized.has_client_secret, config.has_client_secret);
    }

    #[test]
    fn test_oauth_config_clone() {
        let config = OAuthConfig {
            use_custom: true,
            client_id: Some("id".to_string()),
            has_client_secret: true,
        };

        let cloned = config.clone();
        assert_eq!(cloned.use_custom, config.use_custom);
        assert_eq!(cloned.client_id, config.client_id);
    }
}
