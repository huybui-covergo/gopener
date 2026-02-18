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
    #[cfg(test)]
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
#[cfg(test)]
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
#[cfg(test)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_google_file_type_mime_types() {
        assert_eq!(
            GoogleFileType::Document.google_mime_type(),
            "application/vnd.google-apps.document"
        );
        assert_eq!(
            GoogleFileType::Spreadsheet.google_mime_type(),
            "application/vnd.google-apps.spreadsheet"
        );
        assert_eq!(
            GoogleFileType::Presentation.google_mime_type(),
            "application/vnd.google-apps.presentation"
        );
    }

    #[test]
    fn test_google_file_type_display_names() {
        assert_eq!(GoogleFileType::Document.display_name(), "Google Docs");
        assert_eq!(GoogleFileType::Spreadsheet.display_name(), "Google Sheets");
        assert_eq!(GoogleFileType::Presentation.display_name(), "Google Slides");
    }

    #[test]
    fn test_google_file_type_icons() {
        assert_eq!(GoogleFileType::Document.icon(), "📄");
        assert_eq!(GoogleFileType::Spreadsheet.icon(), "📊");
        assert_eq!(GoogleFileType::Presentation.icon(), "📽️");
    }

    #[test]
    fn test_detect_document_types() {
        for ext in &["doc", "docx", "odt", "rtf", "txt"] {
            let filename = format!("test.{}", ext);
            let path = Path::new(&filename);
            assert_eq!(
                detect_file_type(path),
                Some(GoogleFileType::Document),
                "Failed for extension: {}",
                ext
            );
        }
    }

    #[test]
    fn test_detect_spreadsheet_types() {
        for ext in &["xls", "xlsx", "ods", "csv", "tsv"] {
            let filename = format!("test.{}", ext);
            let path = Path::new(&filename);
            assert_eq!(
                detect_file_type(path),
                Some(GoogleFileType::Spreadsheet),
                "Failed for extension: {}",
                ext
            );
        }
    }

    #[test]
    fn test_detect_presentation_types() {
        for ext in &["ppt", "pptx", "odp"] {
            let filename = format!("test.{}", ext);
            let path = Path::new(&filename);
            assert_eq!(
                detect_file_type(path),
                Some(GoogleFileType::Presentation),
                "Failed for extension: {}",
                ext
            );
        }
    }

    #[test]
    fn test_detect_unsupported_types() {
        for ext in &["pdf", "jpg", "png", "zip", "exe", "mp4"] {
            let filename = format!("test.{}", ext);
            let path = Path::new(&filename);
            assert_eq!(
                detect_file_type(path),
                None,
                "Should be None for extension: {}",
                ext
            );
        }
    }

    #[test]
    fn test_detect_no_extension() {
        let path = Path::new("README");
        assert_eq!(detect_file_type(path), None);
    }

    #[test]
    fn test_detect_with_path() {
        let path = Path::new("/home/user/documents/report.docx");
        assert_eq!(detect_file_type(path), Some(GoogleFileType::Document));
    }

    #[test]
    fn test_get_mime_type() {
        assert_eq!(
            get_mime_type(Path::new("test.docx")),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
        );
        assert_eq!(get_mime_type(Path::new("test.txt")), "text/plain");
        assert_eq!(get_mime_type(Path::new("test.csv")), "text/csv");
    }

    #[test]
    fn test_get_mime_type_unknown() {
        assert_eq!(
            get_mime_type(Path::new("test.unknownext")),
            "application/octet-stream"
        );
    }

    #[test]
    fn test_is_supported_extension() {
        assert!(is_supported_extension("doc"));
        assert!(is_supported_extension("docx"));
        assert!(is_supported_extension("xlsx"));
        assert!(is_supported_extension("pptx"));
        assert!(is_supported_extension("csv"));
        assert!(is_supported_extension("odp"));
    }

    #[test]
    fn test_is_supported_extension_case_insensitive() {
        assert!(is_supported_extension("DOCX"));
        assert!(is_supported_extension("Xlsx"));
        assert!(is_supported_extension("PPT"));
    }

    #[test]
    fn test_unsupported_extension() {
        assert!(!is_supported_extension("pdf"));
        assert!(!is_supported_extension("jpg"));
        assert!(!is_supported_extension("zip"));
        assert!(!is_supported_extension("exe"));
    }

    #[test]
    fn test_supported_extensions_list() {
        let exts = supported_extensions();
        assert_eq!(exts.len(), 13);
        assert!(exts.contains(&"doc"));
        assert!(exts.contains(&"docx"));
        assert!(exts.contains(&"xlsx"));
        assert!(exts.contains(&"pptx"));
        assert!(exts.contains(&"csv"));
        assert!(exts.contains(&"odp"));
    }

    #[test]
    fn test_get_file_info_nonexistent() {
        let result = get_file_info(Path::new("/nonexistent/file.docx"));
        assert!(result.is_err());
    }

    #[test]
    fn test_get_file_info_with_temp_file() {
        let dir = std::env::temp_dir();
        let path = dir.join("gopener_test_file.docx");
        std::fs::write(&path, "test content").unwrap();

        let info = get_file_info(&path).unwrap();
        assert_eq!(info.name, "gopener_test_file.docx");
        assert_eq!(info.extension, "docx");
        assert_eq!(info.file_type, Some("Google Docs".to_string()));
        assert_eq!(
            info.google_type,
            Some("application/vnd.google-apps.document".to_string())
        );
        assert!(info.size > 0);

        std::fs::remove_file(&path).unwrap();
    }
}
