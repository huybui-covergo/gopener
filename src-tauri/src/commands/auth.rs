use crate::utils::keychain::{self, keys};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

// Default OAuth credentials (users can override with their own)
const DEFAULT_CLIENT_ID: &str = "YOUR_CLIENT_ID.apps.googleusercontent.com";
const REDIRECT_URI: &str = "http://localhost:8085";

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: u64,
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthState {
    pub is_authenticated: bool,
    pub access_token: Option<String>,
    pub expires_at: Option<u64>,
}

/// Generate PKCE code verifier and challenge
fn generate_pkce() -> (String, String) {
    // Generate random 32 bytes for code verifier
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    let code_verifier = URL_SAFE_NO_PAD.encode(&random_bytes);

    // Generate code challenge (SHA256 hash of verifier, base64url encoded)
    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let hash = hasher.finalize();
    let code_challenge = URL_SAFE_NO_PAD.encode(hash);

    (code_verifier, code_challenge)
}

/// Get OAuth client ID (custom or default)
async fn get_client_id() -> String {
    match keychain::retrieve(keys::CUSTOM_CLIENT_ID) {
        Ok(Some(id)) if !id.is_empty() => id,
        _ => DEFAULT_CLIENT_ID.to_string(),
    }
}

/// Get OAuth client secret (only for custom credentials)
async fn get_client_secret() -> Option<String> {
    match keychain::retrieve(keys::CUSTOM_CLIENT_SECRET) {
        Ok(Some(secret)) if !secret.is_empty() => Some(secret),
        _ => None,
    }
}

/// Generate the OAuth authorization URL
#[tauri::command]
pub async fn get_auth_url() -> Result<String, String> {
    let (code_verifier, code_challenge) = generate_pkce();

    // Store the code verifier for later use
    keychain::store(keys::PKCE_VERIFIER, &code_verifier).map_err(|e| e.to_string())?;

    let client_id = get_client_id().await;

    let scopes = [
        "https://www.googleapis.com/auth/drive.file",
        "https://www.googleapis.com/auth/drive.readonly",
    ]
    .join(" ");

    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?\
        client_id={}&\
        redirect_uri={}&\
        response_type=code&\
        scope={}&\
        code_challenge={}&\
        code_challenge_method=S256&\
        access_type=offline&\
        prompt=consent",
        urlencoding::encode(&client_id),
        urlencoding::encode(REDIRECT_URI),
        urlencoding::encode(&scopes),
        urlencoding::encode(&code_challenge)
    );

    Ok(auth_url)
}

/// Exchange authorization code for tokens
#[tauri::command]
pub async fn exchange_code(code: String) -> Result<AuthState, String> {
    let client_id = get_client_id().await;
    let client_secret = get_client_secret().await;

    // Retrieve the stored code verifier
    let code_verifier = keychain::retrieve(keys::PKCE_VERIFIER)
        .map_err(|e| e.to_string())?
        .ok_or("No PKCE verifier found")?;

    let client = reqwest::Client::new();

    let mut params = vec![
        ("code", code),
        ("client_id", client_id),
        ("redirect_uri", REDIRECT_URI.to_string()),
        ("grant_type", "authorization_code".to_string()),
        ("code_verifier", code_verifier),
    ];

    // Add client secret if using custom credentials
    if let Some(secret) = client_secret {
        params.push(("client_secret", secret));
    }

    let response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("Failed to exchange code: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Token exchange failed: {}", error_text));
    }

    let token_response: TokenResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse token response: {}", e))?;

    // Calculate expiry timestamp
    let expires_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + token_response.expires_in;

    // Store tokens securely
    keychain::store(keys::ACCESS_TOKEN, &token_response.access_token).map_err(|e| e.to_string())?;

    if let Some(ref refresh_token) = token_response.refresh_token {
        keychain::store(keys::REFRESH_TOKEN, refresh_token).map_err(|e| e.to_string())?;
    }

    keychain::store(keys::TOKEN_EXPIRY, &expires_at.to_string()).map_err(|e| e.to_string())?;

    // Clean up PKCE verifier
    let _ = keychain::delete(keys::PKCE_VERIFIER);

    Ok(AuthState {
        is_authenticated: true,
        access_token: Some(token_response.access_token),
        expires_at: Some(expires_at),
    })
}

/// Refresh the access token using the refresh token
#[tauri::command]
pub async fn refresh_token() -> Result<AuthState, String> {
    let client_id = get_client_id().await;
    let client_secret = get_client_secret().await;

    let refresh_token = keychain::retrieve(keys::REFRESH_TOKEN)
        .map_err(|e| e.to_string())?
        .ok_or("No refresh token found")?;

    let client = reqwest::Client::new();

    let mut params = vec![
        ("refresh_token", refresh_token),
        ("client_id", client_id),
        ("grant_type", "refresh_token".to_string()),
    ];

    if let Some(secret) = client_secret {
        params.push(("client_secret", secret));
    }

    let response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("Failed to refresh token: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Token refresh failed: {}", error_text));
    }

    let token_response: TokenResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse token response: {}", e))?;

    let expires_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + token_response.expires_in;

    keychain::store(keys::ACCESS_TOKEN, &token_response.access_token).map_err(|e| e.to_string())?;
    keychain::store(keys::TOKEN_EXPIRY, &expires_at.to_string()).map_err(|e| e.to_string())?;

    Ok(AuthState {
        is_authenticated: true,
        access_token: Some(token_response.access_token),
        expires_at: Some(expires_at),
    })
}

/// Sign out and clear all stored credentials
#[tauri::command]
pub async fn sign_out() -> Result<(), String> {
    let _ = keychain::delete(keys::ACCESS_TOKEN);
    let _ = keychain::delete(keys::REFRESH_TOKEN);
    let _ = keychain::delete(keys::TOKEN_EXPIRY);
    let _ = keychain::delete(keys::PKCE_VERIFIER);
    Ok(())
}

/// Check current authentication state
#[tauri::command]
pub async fn check_auth() -> Result<AuthState, String> {
    let access_token = keychain::retrieve(keys::ACCESS_TOKEN).map_err(|e| e.to_string())?;

    let expires_at = keychain::retrieve(keys::TOKEN_EXPIRY)
        .map_err(|e| e.to_string())?
        .and_then(|s| s.parse::<u64>().ok());

    // Check if token exists and is not expired
    let is_authenticated = match (&access_token, expires_at) {
        (Some(_), Some(expiry)) => {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            // Consider token valid if it has more than 5 minutes left
            expiry > now + 300
        }
        _ => false,
    };

    // If token is expired but we have a refresh token, try to refresh
    if !is_authenticated {
        if let Ok(Some(_)) = keychain::retrieve(keys::REFRESH_TOKEN) {
            // Try to refresh the token
            match refresh_token().await {
                Ok(state) => return Ok(state),
                Err(_) => {
                    // Refresh failed, user needs to re-authenticate
                    return Ok(AuthState {
                        is_authenticated: false,
                        access_token: None,
                        expires_at: None,
                    });
                }
            }
        }
    }

    Ok(AuthState {
        is_authenticated,
        access_token,
        expires_at,
    })
}

/// Get a valid access token, refreshing if necessary
pub async fn get_valid_token() -> Result<String, String> {
    let state = check_auth().await?;

    state
        .access_token
        .ok_or_else(|| "Not authenticated".to_string())
}
