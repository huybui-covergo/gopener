import { describe, it, expect } from "vitest";
import {
  getGoogleTypeFromExtension,
  getGoogleTypeDisplayName,
  getFileTypeIcon,
  isSupportedExtension,
  getAllSupportedExtensions,
  GOOGLE_MIME_TYPES,
  SUPPORTED_EXTENSIONS,
} from "../google-api";

describe("google-api", () => {
  describe("GOOGLE_MIME_TYPES", () => {
    it("has correct MIME types", () => {
      expect(GOOGLE_MIME_TYPES.document).toBe(
        "application/vnd.google-apps.document"
      );
      expect(GOOGLE_MIME_TYPES.spreadsheet).toBe(
        "application/vnd.google-apps.spreadsheet"
      );
      expect(GOOGLE_MIME_TYPES.presentation).toBe(
        "application/vnd.google-apps.presentation"
      );
      expect(GOOGLE_MIME_TYPES.folder).toBe(
        "application/vnd.google-apps.folder"
      );
    });
  });

  describe("SUPPORTED_EXTENSIONS", () => {
    it("has document extensions", () => {
      expect(SUPPORTED_EXTENSIONS.document).toContain("doc");
      expect(SUPPORTED_EXTENSIONS.document).toContain("docx");
      expect(SUPPORTED_EXTENSIONS.document).toContain("odt");
      expect(SUPPORTED_EXTENSIONS.document).toContain("rtf");
      expect(SUPPORTED_EXTENSIONS.document).toContain("txt");
    });

    it("has spreadsheet extensions", () => {
      expect(SUPPORTED_EXTENSIONS.spreadsheet).toContain("xls");
      expect(SUPPORTED_EXTENSIONS.spreadsheet).toContain("xlsx");
      expect(SUPPORTED_EXTENSIONS.spreadsheet).toContain("csv");
    });

    it("has presentation extensions", () => {
      expect(SUPPORTED_EXTENSIONS.presentation).toContain("ppt");
      expect(SUPPORTED_EXTENSIONS.presentation).toContain("pptx");
      expect(SUPPORTED_EXTENSIONS.presentation).toContain("odp");
    });
  });

  describe("getGoogleTypeFromExtension", () => {
    it("returns document for doc extensions", () => {
      expect(getGoogleTypeFromExtension("doc")).toBe("document");
      expect(getGoogleTypeFromExtension("docx")).toBe("document");
      expect(getGoogleTypeFromExtension("odt")).toBe("document");
      expect(getGoogleTypeFromExtension("rtf")).toBe("document");
      expect(getGoogleTypeFromExtension("txt")).toBe("document");
    });

    it("returns spreadsheet for spreadsheet extensions", () => {
      expect(getGoogleTypeFromExtension("xls")).toBe("spreadsheet");
      expect(getGoogleTypeFromExtension("xlsx")).toBe("spreadsheet");
      expect(getGoogleTypeFromExtension("ods")).toBe("spreadsheet");
      expect(getGoogleTypeFromExtension("csv")).toBe("spreadsheet");
      expect(getGoogleTypeFromExtension("tsv")).toBe("spreadsheet");
    });

    it("returns presentation for presentation extensions", () => {
      expect(getGoogleTypeFromExtension("ppt")).toBe("presentation");
      expect(getGoogleTypeFromExtension("pptx")).toBe("presentation");
      expect(getGoogleTypeFromExtension("odp")).toBe("presentation");
    });

    it("returns null for unsupported extensions", () => {
      expect(getGoogleTypeFromExtension("pdf")).toBeNull();
      expect(getGoogleTypeFromExtension("jpg")).toBeNull();
      expect(getGoogleTypeFromExtension("zip")).toBeNull();
    });

    it("handles extensions with leading dot", () => {
      expect(getGoogleTypeFromExtension(".docx")).toBe("document");
      expect(getGoogleTypeFromExtension(".xlsx")).toBe("spreadsheet");
    });

    it("is case-insensitive", () => {
      expect(getGoogleTypeFromExtension("DOCX")).toBe("document");
      expect(getGoogleTypeFromExtension("Xlsx")).toBe("spreadsheet");
      expect(getGoogleTypeFromExtension("PPTX")).toBe("presentation");
    });
  });

  describe("getGoogleTypeDisplayName", () => {
    it("returns correct display names", () => {
      expect(getGoogleTypeDisplayName("document")).toBe("Google Docs");
      expect(getGoogleTypeDisplayName("spreadsheet")).toBe("Google Sheets");
      expect(getGoogleTypeDisplayName("presentation")).toBe("Google Slides");
      expect(getGoogleTypeDisplayName("folder")).toBe("Google Drive Folder");
    });
  });

  describe("getFileTypeIcon", () => {
    it("returns correct icons for type names", () => {
      expect(getFileTypeIcon("Google Docs")).toBe("📄");
      expect(getFileTypeIcon("document")).toBe("📄");
      expect(getFileTypeIcon("Google Sheets")).toBe("📊");
      expect(getFileTypeIcon("spreadsheet")).toBe("📊");
      expect(getFileTypeIcon("Google Slides")).toBe("📽️");
      expect(getFileTypeIcon("presentation")).toBe("📽️");
      expect(getFileTypeIcon("folder")).toBe("📁");
    });

    it("returns default icon for unknown types", () => {
      expect(getFileTypeIcon("unknown")).toBe("📄");
    });
  });

  describe("isSupportedExtension", () => {
    it("returns true for supported extensions", () => {
      expect(isSupportedExtension("docx")).toBe(true);
      expect(isSupportedExtension("xlsx")).toBe(true);
      expect(isSupportedExtension("pptx")).toBe(true);
      expect(isSupportedExtension("csv")).toBe(true);
      expect(isSupportedExtension("txt")).toBe(true);
    });

    it("returns false for unsupported extensions", () => {
      expect(isSupportedExtension("pdf")).toBe(false);
      expect(isSupportedExtension("jpg")).toBe(false);
      expect(isSupportedExtension("mp4")).toBe(false);
    });

    it("handles leading dots", () => {
      expect(isSupportedExtension(".docx")).toBe(true);
    });

    it("is case-insensitive", () => {
      expect(isSupportedExtension("DOCX")).toBe(true);
      expect(isSupportedExtension("Xlsx")).toBe(true);
    });
  });

  describe("getAllSupportedExtensions", () => {
    it("returns all extensions as a flat array", () => {
      const extensions = getAllSupportedExtensions();

      expect(extensions).toContain("doc");
      expect(extensions).toContain("xlsx");
      expect(extensions).toContain("pptx");
      expect(extensions).toContain("csv");
      expect(extensions).toContain("odp");
    });

    it("returns the correct total count", () => {
      const extensions = getAllSupportedExtensions();
      const expectedCount =
        SUPPORTED_EXTENSIONS.document.length +
        SUPPORTED_EXTENSIONS.spreadsheet.length +
        SUPPORTED_EXTENSIONS.presentation.length;

      expect(extensions).toHaveLength(expectedCount);
    });
  });
});
