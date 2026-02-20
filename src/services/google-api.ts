// Google API wrapper for frontend
// Most Google API calls are handled by the Rust backend
// This file provides TypeScript types and any frontend-specific helpers

export interface GoogleFile {
  id: string;
  name: string;
  mimeType: string;
  webViewLink?: string;
  parents?: string[];
}

export interface GoogleFolder {
  id: string;
  name: string;
  mimeType: string;
}

// File type mappings
export const GOOGLE_MIME_TYPES = {
  document: "application/vnd.google-apps.document",
  spreadsheet: "application/vnd.google-apps.spreadsheet",
  presentation: "application/vnd.google-apps.presentation",
  folder: "application/vnd.google-apps.folder",
} as const;

// Supported file extensions
export const SUPPORTED_EXTENSIONS = {
  document: ["doc", "docx", "odt", "rtf", "txt"],
  spreadsheet: ["xls", "xlsx", "ods", "csv", "tsv"],
  presentation: ["ppt", "pptx", "odp"],
} as const;

// Get Google type from file extension
export function getGoogleTypeFromExtension(
  extension: string
): keyof typeof GOOGLE_MIME_TYPES | null {
  const ext = extension.toLowerCase().replace(".", "");

  if (SUPPORTED_EXTENSIONS.document.includes(ext as any)) {
    return "document";
  }
  if (SUPPORTED_EXTENSIONS.spreadsheet.includes(ext as any)) {
    return "spreadsheet";
  }
  if (SUPPORTED_EXTENSIONS.presentation.includes(ext as any)) {
    return "presentation";
  }

  return null;
}

// Get display name for Google type
export function getGoogleTypeDisplayName(
  type: keyof typeof GOOGLE_MIME_TYPES
): string {
  switch (type) {
    case "document":
      return "Google Docs";
    case "spreadsheet":
      return "Google Sheets";
    case "presentation":
      return "Google Slides";
    case "folder":
      return "Google Drive Folder";
    default:
      return "Google Drive";
  }
}

// Get icon for file type
export function getFileTypeIcon(type: string): string {
  switch (type) {
    case "Google Docs":
    case "document":
      return "📄";
    case "Google Sheets":
    case "spreadsheet":
      return "📊";
    case "Google Slides":
    case "presentation":
      return "📽️";
    case "folder":
      return "📁";
    default:
      return "📄";
  }
}

// Check if extension is supported
export function isSupportedExtension(extension: string): boolean {
  const ext = extension.toLowerCase().replace(".", "");
  return (
    SUPPORTED_EXTENSIONS.document.includes(ext as any) ||
    SUPPORTED_EXTENSIONS.spreadsheet.includes(ext as any) ||
    SUPPORTED_EXTENSIONS.presentation.includes(ext as any)
  );
}

// Get all supported extensions as a flat array
export function getAllSupportedExtensions(): string[] {
  return [
    ...SUPPORTED_EXTENSIONS.document,
    ...SUPPORTED_EXTENSIONS.spreadsheet,
    ...SUPPORTED_EXTENSIONS.presentation,
  ];
}
