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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_url_document() {
        let url = get_file_url("abc123", "application/vnd.google-apps.document");
        assert_eq!(url, "https://docs.google.com/document/d/abc123/edit");
    }

    #[test]
    fn test_get_file_url_spreadsheet() {
        let url = get_file_url("xyz789", "application/vnd.google-apps.spreadsheet");
        assert_eq!(url, "https://docs.google.com/spreadsheets/d/xyz789/edit");
    }

    #[test]
    fn test_get_file_url_presentation() {
        let url = get_file_url("pres456", "application/vnd.google-apps.presentation");
        assert_eq!(
            url,
            "https://docs.google.com/presentation/d/pres456/edit"
        );
    }

    #[test]
    fn test_get_file_url_unknown_type() {
        let url = get_file_url("file789", "application/pdf");
        assert_eq!(url, "https://drive.google.com/file/d/file789/view");
    }

    #[test]
    fn test_get_file_url_empty_type() {
        let url = get_file_url("file000", "");
        assert_eq!(url, "https://drive.google.com/file/d/file000/view");
    }

    #[test]
    fn test_drive_folder_serialization() {
        let folder = DriveFolder {
            id: "folder-1".to_string(),
            name: "My Folder".to_string(),
            mime_type: "application/vnd.google-apps.folder".to_string(),
        };

        let json = serde_json::to_string(&folder).unwrap();
        assert!(json.contains("\"id\":\"folder-1\""));
        assert!(json.contains("\"name\":\"My Folder\""));
        assert!(json.contains("\"mimeType\""));
    }

    #[test]
    fn test_drive_folder_deserialization() {
        let json = r#"{"id":"f1","name":"Test","mimeType":"application/vnd.google-apps.folder"}"#;
        let folder: DriveFolder = serde_json::from_str(json).unwrap();

        assert_eq!(folder.id, "f1");
        assert_eq!(folder.name, "Test");
        assert_eq!(folder.mime_type, "application/vnd.google-apps.folder");
    }
}
