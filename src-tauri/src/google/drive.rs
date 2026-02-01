use crate::google::client::GoogleClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DriveFolder {
    pub id: String,
    pub name: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
}

#[derive(Debug, Deserialize)]
struct FolderListResponse {
    files: Vec<DriveFolder>,
    #[serde(rename = "nextPageToken")]
    next_page_token: Option<String>,
}

/// List folders in Google Drive
#[tauri::command]
pub async fn list_folders(parent_id: Option<String>) -> Result<Vec<DriveFolder>, String> {
    let client = GoogleClient::new().await?;

    // Build query to get folders only
    let mut query = "mimeType='application/vnd.google-apps.folder' and trashed=false".to_string();

    if let Some(ref parent) = parent_id {
        query = format!("{} and '{}' in parents", query, parent);
    } else {
        // Root level - get items in "My Drive"
        query = format!("{} and 'root' in parents", query);
    }

    let endpoint = format!(
        "/files?q={}&fields=files(id,name,mimeType)&orderBy=name&pageSize=100",
        urlencoding::encode(&query)
    );

    let response: FolderListResponse = client.get(&endpoint).await?;

    Ok(response.files)
}

/// Create a new folder in Google Drive
#[tauri::command]
pub async fn create_folder(name: String, parent_id: Option<String>) -> Result<DriveFolder, String> {
    let client = GoogleClient::new().await?;

    let mut metadata = serde_json::json!({
        "name": name,
        "mimeType": "application/vnd.google-apps.folder"
    });

    if let Some(parent) = parent_id {
        metadata["parents"] = serde_json::json!([parent]);
    }

    let folder: DriveFolder = client
        .post("/files?fields=id,name,mimeType", &metadata)
        .await?;

    Ok(folder)
}

/// Get the web URL for a file
pub fn get_file_url(file_id: &str, file_type: &str) -> String {
    match file_type {
        "application/vnd.google-apps.document" => {
            format!("https://docs.google.com/document/d/{}/edit", file_id)
        }
        "application/vnd.google-apps.spreadsheet" => {
            format!("https://docs.google.com/spreadsheets/d/{}/edit", file_id)
        }
        "application/vnd.google-apps.presentation" => {
            format!("https://docs.google.com/presentation/d/{}/edit", file_id)
        }
        _ => format!("https://drive.google.com/file/d/{}/view", file_id),
    }
}
