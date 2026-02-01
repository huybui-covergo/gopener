use keyring::Entry;
use thiserror::Error;

const SERVICE_NAME: &str = "gopener";

#[derive(Error, Debug)]
pub enum KeychainError {
    #[error("Keychain error: {0}")]
    Keyring(#[from] keyring::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, KeychainError>;

/// Store a value in the system keychain
pub fn store(key: &str, value: &str) -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, key)?;
    entry.set_password(value)?;
    Ok(())
}

/// Retrieve a value from the system keychain
pub fn retrieve(key: &str) -> Result<Option<String>> {
    let entry = Entry::new(SERVICE_NAME, key)?;
    match entry.get_password() {
        Ok(value) => Ok(Some(value)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(KeychainError::Keyring(e)),
    }
}

/// Delete a value from the system keychain
pub fn delete(key: &str) -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, key)?;
    match entry.delete_credential() {
        Ok(()) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()), // Already deleted
        Err(e) => Err(KeychainError::Keyring(e)),
    }
}

/// Store a JSON-serializable value
pub fn store_json<T: serde::Serialize>(key: &str, value: &T) -> Result<()> {
    let json = serde_json::to_string(value)?;
    store(key, &json)
}

/// Retrieve and deserialize a JSON value
pub fn retrieve_json<T: serde::de::DeserializeOwned>(key: &str) -> Result<Option<T>> {
    match retrieve(key)? {
        Some(json) => {
            let value = serde_json::from_str(&json)?;
            Ok(Some(value))
        }
        None => Ok(None),
    }
}

// Key constants for different stored values
pub mod keys {
    pub const ACCESS_TOKEN: &str = "access_token";
    pub const REFRESH_TOKEN: &str = "refresh_token";
    pub const TOKEN_EXPIRY: &str = "token_expiry";
    pub const CUSTOM_CLIENT_ID: &str = "custom_client_id";
    pub const CUSTOM_CLIENT_SECRET: &str = "custom_client_secret";
    pub const PKCE_VERIFIER: &str = "pkce_verifier";
}
