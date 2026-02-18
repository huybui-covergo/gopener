use crate::commands::auth::get_valid_token;
use crate::utils::file::{detect_file_type, get_file_info, get_mime_type};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use tauri::Emitter;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UploadResult {
    pub file_id: String,
    pub name: String,
    pub web_view_link: String,
    pub file_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UploadProgress {
    pub bytes_uploaded: u64,
    pub total_bytes: u64,
    pub percentage: f64,
}

// Global progress tracking (simplified for single file uploads)
static UPLOAD_PROGRESS: AtomicU64 = AtomicU64::new(0);
static UPLOAD_TOTAL: AtomicU64 = AtomicU64::new(0);

/// Upload a file to Google Drive with conversion
#[tauri::command]
pub async fn upload_file(
    app: tauri::AppHandle,
    file_path: String,
    folder_id: Option<String>,
) -> Result<UploadResult, String> {
    let path = Path::new(&file_path);

    // Validate file exists
    if !path.exists() {
        return Err("File does not exist".to_string());
    }

    // Get file info
    let file_info = get_file_info(path).map_err(|e| format!("Failed to get file info: {}", e))?;

    // Detect file type for conversion
    let google_type = detect_file_type(path).ok_or("Unsupported file type")?;

    // Get valid access token
    let access_token = get_valid_token().await?;

    // Read file contents
    let file_contents =
        std::fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;

    let total_size = file_contents.len() as u64;
    UPLOAD_TOTAL.store(total_size, Ordering::SeqCst);
    UPLOAD_PROGRESS.store(0, Ordering::SeqCst);

    // Emit initial progress
    let _ = app.emit(
        "upload-progress",
        UploadProgress {
            bytes_uploaded: 0,
            total_bytes: total_size,
            percentage: 0.0,
        },
    );

    // Build metadata
    let mut metadata = serde_json::json!({
        "name": file_info.name,
        "mimeType": google_type.google_mime_type(),
    });

    // Add parent folder if specified
    if let Some(ref folder) = folder_id {
        metadata["parents"] = serde_json::json!([folder]);
    }

    // Create multipart form
    let metadata_json = serde_json::to_string(&metadata).map_err(|e| e.to_string())?;

    let file_mime = get_mime_type(path);

    // Build multipart body manually for Google Drive API
    let boundary = "gopener_boundary_12345";
    let mut body = Vec::new();

    // Metadata part
    body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    body.extend_from_slice(b"Content-Type: application/json; charset=UTF-8\r\n\r\n");
    body.extend_from_slice(metadata_json.as_bytes());
    body.extend_from_slice(b"\r\n");

    // File part
    body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    body.extend_from_slice(format!("Content-Type: {}\r\n\r\n", file_mime).as_bytes());
    body.extend_from_slice(&file_contents);
    body.extend_from_slice(b"\r\n");

    // End boundary
    body.extend_from_slice(format!("--{}--", boundary).as_bytes());

    let client = reqwest::Client::new();

    let response = client
        .post("https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart&fields=id,name,webViewLink,mimeType")
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", format!("multipart/related; boundary={}", boundary))
        .body(body)
        .send()
        .await
        .map_err(|e| format!("Upload failed: {}", e))?;

    // Update progress to 100%
    UPLOAD_PROGRESS.store(total_size, Ordering::SeqCst);
    let _ = app.emit(
        "upload-progress",
        UploadProgress {
            bytes_uploaded: total_size,
            total_bytes: total_size,
            percentage: 100.0,
        },
    );

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Upload failed: {}", error_text));
    }

    #[derive(Deserialize)]
    struct DriveFile {
        id: String,
        name: String,
        #[serde(rename = "webViewLink")]
        web_view_link: String,
        #[serde(rename = "mimeType")]
        _mime_type: String,
    }

    let drive_file: DriveFile = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(UploadResult {
        file_id: drive_file.id,
        name: drive_file.name,
        web_view_link: drive_file.web_view_link,
        file_type: google_type.display_name().to_string(),
    })
}

/// Get current upload progress
#[tauri::command]
pub async fn get_upload_progress() -> UploadProgress {
    let uploaded = UPLOAD_PROGRESS.load(Ordering::SeqCst);
    let total = UPLOAD_TOTAL.load(Ordering::SeqCst);
    let percentage = if total > 0 {
        (uploaded as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    UploadProgress {
        bytes_uploaded: uploaded,
        total_bytes: total,
        percentage,
    }
}
