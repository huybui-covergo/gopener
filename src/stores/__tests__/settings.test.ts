import { describe, it, expect, beforeEach, vi } from "vitest";
import { setActivePinia, createPinia } from "pinia";
import { useSettingsStore } from "../settings";
import { invoke } from "@tauri-apps/api/core";

vi.mock("@tauri-apps/api/core");

const mockedInvoke = vi.mocked(invoke);

describe("useSettingsStore", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.clearAllMocks();
  });

  it("has correct initial state", () => {
    const store = useSettingsStore();

    expect(store.defaultFolderId).toBeNull();
    expect(store.defaultFolderName).toBeNull();
    expect(store.autoOpenAfterUpload).toBe(true);
    expect(store.autoCloseAfterUpload).toBe(false);
    expect(store.theme).toBe("dark");
    expect(store.recentFiles).toEqual([]);
    expect(store.isLoading).toBe(false);
  });

  describe("loadSettings", () => {
    it("loads settings from backend", async () => {
      const store = useSettingsStore();
      const mockSettings = {
        default_folder_id: "folder-123",
        default_folder_name: "My Folder",
        auto_open_after_upload: false,
        auto_close_after_upload: true,
        theme: "light",
        recent_files: [
          {
            id: "file-1",
            name: "test.docx",
            path: "/path/to/test.docx",
            google_url: "https://docs.google.com/document/d/abc",
            file_type: "Google Docs",
            uploaded_at: 1700000000,
          },
        ],
      };
      mockedInvoke.mockResolvedValueOnce(mockSettings);

      await store.loadSettings();

      expect(mockedInvoke).toHaveBeenCalledWith("get_settings");
      expect(store.defaultFolderId).toBe("folder-123");
      expect(store.defaultFolderName).toBe("My Folder");
      expect(store.autoOpenAfterUpload).toBe(false);
      expect(store.autoCloseAfterUpload).toBe(true);
      expect(store.theme).toBe("light");
      expect(store.recentFiles).toHaveLength(1);
      expect(store.isLoading).toBe(false);
    });

    it("defaults theme to dark when empty", async () => {
      const store = useSettingsStore();
      mockedInvoke.mockResolvedValueOnce({
        default_folder_id: null,
        default_folder_name: null,
        auto_open_after_upload: true,
        auto_close_after_upload: false,
        theme: "",
        recent_files: [],
      });

      await store.loadSettings();

      expect(store.theme).toBe("dark");
    });

    it("handles load failure gracefully", async () => {
      const store = useSettingsStore();
      const consoleSpy = vi.spyOn(console, "error").mockImplementation(() => {});
      mockedInvoke.mockRejectedValueOnce(new Error("Disk error"));

      await store.loadSettings();

      expect(consoleSpy).toHaveBeenCalled();
      expect(store.isLoading).toBe(false);
      consoleSpy.mockRestore();
    });
  });

  describe("saveSettings", () => {
    it("saves current state to backend", async () => {
      const store = useSettingsStore();
      store.defaultFolderId = "folder-abc";
      store.theme = "light";
      mockedInvoke.mockResolvedValueOnce(undefined);

      await store.saveSettings();

      expect(mockedInvoke).toHaveBeenCalledWith("save_settings", {
        settings: {
          default_folder_id: "folder-abc",
          default_folder_name: null,
          auto_open_after_upload: true,
          auto_close_after_upload: false,
          theme: "light",
          recent_files: [],
        },
      });
    });
  });

  describe("setDefaultFolder", () => {
    it("updates folder and saves", async () => {
      const store = useSettingsStore();
      mockedInvoke.mockResolvedValue(undefined);

      await store.setDefaultFolder("new-folder", "New Folder");

      expect(store.defaultFolderId).toBe("new-folder");
      expect(store.defaultFolderName).toBe("New Folder");
      expect(mockedInvoke).toHaveBeenCalledWith("save_settings", expect.any(Object));
    });
  });

  describe("setAutoOpenAfterUpload", () => {
    it("updates preference and saves", async () => {
      const store = useSettingsStore();
      mockedInvoke.mockResolvedValue(undefined);

      await store.setAutoOpenAfterUpload(false);

      expect(store.autoOpenAfterUpload).toBe(false);
      expect(mockedInvoke).toHaveBeenCalled();
    });
  });

  describe("setAutoCloseAfterUpload", () => {
    it("updates preference and saves", async () => {
      const store = useSettingsStore();
      mockedInvoke.mockResolvedValue(undefined);

      await store.setAutoCloseAfterUpload(true);

      expect(store.autoCloseAfterUpload).toBe(true);
    });
  });

  describe("setTheme", () => {
    it("updates theme and saves", async () => {
      const store = useSettingsStore();
      mockedInvoke.mockResolvedValue(undefined);

      await store.setTheme("light");

      expect(store.theme).toBe("light");
    });
  });

  describe("addRecentFile", () => {
    it("invokes backend and reloads settings", async () => {
      const store = useSettingsStore();
      const file = {
        id: "file-1",
        name: "test.docx",
        path: "/path/test.docx",
        google_url: "https://docs.google.com/document/d/abc",
        file_type: "Google Docs",
        uploaded_at: 1700000000,
      };

      mockedInvoke
        .mockResolvedValueOnce(undefined) // add_recent_file
        .mockResolvedValueOnce({
          // get_settings (reload)
          default_folder_id: null,
          default_folder_name: null,
          auto_open_after_upload: true,
          auto_close_after_upload: false,
          theme: "dark",
          recent_files: [file],
        });

      await store.addRecentFile(file);

      expect(mockedInvoke).toHaveBeenCalledWith("add_recent_file", { file });
      expect(store.recentFiles).toHaveLength(1);
    });
  });

  describe("clearRecentFiles", () => {
    it("clears recent files", async () => {
      const store = useSettingsStore();
      store.recentFiles = [
        {
          id: "1",
          name: "a.docx",
          path: "/a.docx",
          google_url: "https://...",
          file_type: "Google Docs",
          uploaded_at: 1700000000,
        },
      ];
      mockedInvoke.mockResolvedValueOnce(undefined);

      await store.clearRecentFiles();

      expect(mockedInvoke).toHaveBeenCalledWith("clear_recent_files");
      expect(store.recentFiles).toEqual([]);
    });
  });
});
