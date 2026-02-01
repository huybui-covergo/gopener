use crate::commands::auth::get_valid_token;
use reqwest::Client;
use serde::de::DeserializeOwned;

const DRIVE_API_BASE: &str = "https://www.googleapis.com/drive/v3";

pub struct GoogleClient {
    client: Client,
    access_token: String,
}

impl GoogleClient {
    pub async fn new() -> Result<Self, String> {
        let access_token = get_valid_token().await?;
        Ok(Self {
            client: Client::new(),
            access_token,
        })
    }

    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, String> {
        let url = format!("{}{}", DRIVE_API_BASE, endpoint);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("API error: {}", error_text));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    pub async fn post<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &serde_json::Value,
    ) -> Result<T, String> {
        let url = format!("{}{}", DRIVE_API_BASE, endpoint);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("API error: {}", error_text));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }
}
