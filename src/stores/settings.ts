import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export interface RecentFile {
  id: string;
  name: string;
  path: string;
  google_url: string;
  file_type: string;
  uploaded_at: number;
}

export interface Settings {
  default_folder_id: string | null;
  default_folder_name: string | null;
  auto_open_after_upload: boolean;
  auto_close_after_upload: boolean;
  theme: string;
  recent_files: RecentFile[];
}

export const useSettingsStore = defineStore("settings", () => {
  const defaultFolderId = ref<string | null>(null);
  const defaultFolderName = ref<string | null>(null);
  const autoOpenAfterUpload = ref(true);
  const autoCloseAfterUpload = ref(false);
  const theme = ref("dark");
  const recentFiles = ref<RecentFile[]>([]);
  const isLoading = ref(false);

  async function loadSettings() {
    isLoading.value = true;
    try {
      const settings = await invoke<Settings>("get_settings");
      defaultFolderId.value = settings.default_folder_id;
      defaultFolderName.value = settings.default_folder_name;
      autoOpenAfterUpload.value = settings.auto_open_after_upload;
      autoCloseAfterUpload.value = settings.auto_close_after_upload;
      theme.value = settings.theme || "dark";
      recentFiles.value = settings.recent_files || [];
    } catch (e) {
      console.error("Failed to load settings:", e);
    } finally {
      isLoading.value = false;
    }
  }

  async function saveSettings() {
    try {
      await invoke("save_settings", {
        settings: {
          default_folder_id: defaultFolderId.value,
          default_folder_name: defaultFolderName.value,
          auto_open_after_upload: autoOpenAfterUpload.value,
          auto_close_after_upload: autoCloseAfterUpload.value,
          theme: theme.value,
          recent_files: recentFiles.value,
        },
      });
    } catch (e) {
      console.error("Failed to save settings:", e);
    }
  }

  async function setDefaultFolder(id: string | null, name: string | null) {
    defaultFolderId.value = id;
    defaultFolderName.value = name;
    await saveSettings();
  }

  async function setAutoOpenAfterUpload(value: boolean) {
    autoOpenAfterUpload.value = value;
    await saveSettings();
  }

  async function setAutoCloseAfterUpload(value: boolean) {
    autoCloseAfterUpload.value = value;
    await saveSettings();
  }

  async function setTheme(value: string) {
    theme.value = value;
    await saveSettings();
  }

  async function addRecentFile(file: RecentFile) {
    try {
      await invoke("add_recent_file", { file });
      // Reload to get updated list
      await loadSettings();
    } catch (e) {
      console.error("Failed to add recent file:", e);
    }
  }

  async function clearRecentFiles() {
    try {
      await invoke("clear_recent_files");
      recentFiles.value = [];
    } catch (e) {
      console.error("Failed to clear recent files:", e);
    }
  }

  return {
    defaultFolderId,
    defaultFolderName,
    autoOpenAfterUpload,
    autoCloseAfterUpload,
    theme,
    recentFiles,
    isLoading,
    loadSettings,
    saveSettings,
    setDefaultFolder,
    setAutoOpenAfterUpload,
    setAutoCloseAfterUpload,
    setTheme,
    addRecentFile,
    clearRecentFiles,
  };
});
