import { describe, it, expect, beforeEach, vi } from "vitest";
import { setActivePinia, createPinia } from "pinia";
import { useUploadStore } from "../upload";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

vi.mock("@tauri-apps/api/core");
vi.mock("@tauri-apps/api/event");

const mockedInvoke = vi.mocked(invoke);
const mockedListen = vi.mocked(listen);

describe("useUploadStore", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.clearAllMocks();
    mockedListen.mockResolvedValue(vi.fn());
  });

  it("has correct initial state", () => {
    const store = useUploadStore();

    expect(store.filePath).toBeNull();
    expect(store.fileName).toBeNull();
    expect(store.fileType).toBeNull();
    expect(store.fileSize).toBe(0);
    expect(store.status).toBe("idle");
    expect(store.progress).toEqual({
      bytes_uploaded: 0,
      total_bytes: 0,
      percentage: 0,
    });
    expect(store.result).toBeNull();
    expect(store.error).toBeNull();
  });

  it("computed properties reflect status correctly", () => {
    const store = useUploadStore();

    expect(store.isUploading).toBe(false);
    expect(store.isComplete).toBe(false);
    expect(store.hasError).toBe(false);

    store.status = "uploading";
    expect(store.isUploading).toBe(true);

    store.status = "success";
    expect(store.isComplete).toBe(true);

    store.status = "error";
    expect(store.hasError).toBe(true);
  });

  describe("setFile", () => {
    it("parses file path and detects document type", () => {
      const store = useUploadStore();

      store.setFile("/home/user/report.docx");

      expect(store.filePath).toBe("/home/user/report.docx");
      expect(store.fileName).toBe("report.docx");
      expect(store.fileType).toBe("Google Docs");
      expect(store.status).toBe("idle");
    });

    it("detects spreadsheet type", () => {
      const store = useUploadStore();

      store.setFile("C:\\Users\\test\\data.xlsx");

      expect(store.fileName).toBe("data.xlsx");
      expect(store.fileType).toBe("Google Sheets");
    });

    it("detects presentation type", () => {
      const store = useUploadStore();

      store.setFile("/presentations/slides.pptx");

      expect(store.fileName).toBe("slides.pptx");
      expect(store.fileType).toBe("Google Slides");
    });

    it("handles .doc extension", () => {
      const store = useUploadStore();
      store.setFile("/test/old.doc");
      expect(store.fileType).toBe("Google Docs");
    });

    it("handles .csv extension", () => {
      const store = useUploadStore();
      store.setFile("/data/export.csv");
      expect(store.fileType).toBe("Google Sheets");
    });

    it("handles .odp extension", () => {
      const store = useUploadStore();
      store.setFile("/slides/deck.odp");
      expect(store.fileType).toBe("Google Slides");
    });

    it("resets previous state when setting new file", () => {
      const store = useUploadStore();
      store.status = "error";
      store.error = "Previous error";
      store.result = {
        file_id: "old",
        name: "old.docx",
        web_view_link: "https://...",
        file_type: "doc",
      };

      store.setFile("/new/file.docx");

      expect(store.status).toBe("idle");
      expect(store.error).toBeNull();
      expect(store.result).toBeNull();
      expect(store.progress).toEqual({
        bytes_uploaded: 0,
        total_bytes: 0,
        percentage: 0,
      });
    });
  });

  describe("upload", () => {
    it("uploads file successfully", async () => {
      const store = useUploadStore();
      store.setFile("/test/report.docx");

      const mockResult = {
        file_id: "abc123",
        name: "report",
        web_view_link: "https://docs.google.com/document/d/abc123/edit",
        file_type: "Google Docs",
      };
      mockedInvoke.mockResolvedValueOnce(mockResult);

      await store.upload("folder-id");

      expect(mockedInvoke).toHaveBeenCalledWith("upload_file", {
        filePath: "/test/report.docx",
        folderId: "folder-id",
      });
      expect(store.result).toEqual(mockResult);
      expect(store.status).toBe("success");
    });

    it("sets error when no file is selected", async () => {
      const store = useUploadStore();

      await store.upload();

      expect(store.error).toBe("No file selected");
      expect(store.status).toBe("error");
    });

    it("handles upload failure", async () => {
      const store = useUploadStore();
      store.setFile("/test/report.docx");
      mockedInvoke.mockRejectedValueOnce(new Error("Upload failed"));

      await store.upload();

      expect(store.error).toBe("Error: Upload failed");
      expect(store.status).toBe("error");
    });

    it("passes null folderId when not provided", async () => {
      const store = useUploadStore();
      store.setFile("/test/file.xlsx");
      mockedInvoke.mockResolvedValueOnce({
        file_id: "x",
        name: "file",
        web_view_link: "https://...",
        file_type: "Google Sheets",
      });

      await store.upload();

      expect(mockedInvoke).toHaveBeenCalledWith("upload_file", {
        filePath: "/test/file.xlsx",
        folderId: null,
      });
    });
  });

  describe("reset", () => {
    it("resets all state to initial values", () => {
      const store = useUploadStore();
      store.setFile("/test/file.docx");
      store.status = "success";
      store.error = "some error";

      store.reset();

      expect(store.filePath).toBeNull();
      expect(store.fileName).toBeNull();
      expect(store.fileType).toBeNull();
      expect(store.fileSize).toBe(0);
      expect(store.status).toBe("idle");
      expect(store.result).toBeNull();
      expect(store.error).toBeNull();
    });
  });
});
