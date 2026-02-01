use std::path::Path;

/// Supported file types and their Google conversion targets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GoogleFileType {
    Document,
    Spreadsheet,
    Presentation,
}

impl GoogleFileType {
    /// Get the Google MIME type for conversion
    pub fn google_mime_type(&self) -> &'static str {
        match self {
            GoogleFileType::Document => "application/vnd.google-apps.document",
            GoogleFileType::Spreadsheet => "application/vnd.google-apps.spreadsheet",
            GoogleFileType::Presentation => "application/vnd.google-apps.presentation",
        }
    }

    /// Get the display name for this file type
    pub fn display_name(&self) -> &'static str {
        match self {
            GoogleFileType::Document => "Google Docs",
            GoogleFileType::Spreadsheet => "Google Sheets",
            GoogleFileType::Presentation => "Google Slides",
        }
    }

    /// Get the icon/emoji for this file type
    pub fn icon(&self) -> &'static str {
        match self {
            GoogleFileType::Document => "📄",
            GoogleFileType::Spreadsheet => "📊",
            GoogleFileType::Presentation => "📽️",
        }
    }
}

/// Detect the file type from a file path
pub fn detect_file_type(path: &Path) -> Option<GoogleFileType> {
    let extension = path.extension()?.to_str()?.to_lowercase();

    match extension.as_str() {
        // Google Docs
        "doc" | "docx" | "odt" | "rtf" | "txt" => Some(GoogleFileType::Document),
        // Google Sheets
        "xls" | "xlsx" | "ods" | "csv" | "tsv" => Some(GoogleFileType::Spreadsheet),
        // Google Slides
        "ppt" | "pptx" | "odp" => Some(GoogleFileType::Presentation),
        _ => None,
    }
}

/// Get the MIME type for a file based on its extension
pub fn get_mime_type(path: &Path) -> String {
    mime_guess::from_path(path)
        .first_or_octet_stream()
        .to_string()
}

/// Check if a file extension is supported
pub fn is_supported_extension(extension: &str) -> bool {
    let ext = extension.to_lowercase();
    matches!(
        ext.as_str(),
        "doc" | "docx" | "odt" | "rtf" | "txt" | "xls" | "xlsx" | "ods" | "csv" | "tsv" | "ppt"
            | "pptx"
            | "odp"
    )
}

/// Get all supported extensions as a list
pub fn supported_extensions() -> Vec<&'static str> {
    vec![
        // Documents
        "doc", "docx", "odt", "rtf", "txt", // Spreadsheets
        "xls", "xlsx", "ods", "csv", "tsv", // Presentations
        "ppt", "pptx", "odp",
    ]
}

/// Get file info from a path
#[derive(Debug, Clone, serde::Serialize)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub extension: String,
    pub size: u64,
    pub file_type: Option<String>,
    pub google_type: Option<String>,
}

pub fn get_file_info(path: &Path) -> std::io::Result<FileInfo> {
    let metadata = std::fs::metadata(path)?;
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let extension = path
        .extension()
        .map(|e| e.to_string_lossy().to_string())
        .unwrap_or_default();
    let file_type = detect_file_type(path);

    Ok(FileInfo {
        path: path.to_string_lossy().to_string(),
        name,
        extension,
        size: metadata.len(),
        file_type: file_type.map(|t| t.display_name().to_string()),
        google_type: file_type.map(|t| t.google_mime_type().to_string()),
    })
}
