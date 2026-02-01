import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export interface UploadProgress {
  bytes_uploaded: number;
  total_bytes: number;
  percentage: number;
}

export interface UploadResult {
  file_id: string;
  name: string;
  web_view_link: string;
  file_type: string;
}

export type UploadStatus = "idle" | "uploading" | "success" | "error";

export const useUploadStore = defineStore("upload", () => {
  const filePath = ref<string | null>(null);
  const fileName = ref<string | null>(null);
  const fileType = ref<string | null>(null);
  const fileSize = ref<number>(0);

  const status = ref<UploadStatus>("idle");
  const progress = ref<UploadProgress>({
    bytes_uploaded: 0,
    total_bytes: 0,
    percentage: 0,
  });
  const result = ref<UploadResult | null>(null);
  const error = ref<string | null>(null);

  const isUploading = computed(() => status.value === "uploading");
  const isComplete = computed(() => status.value === "success");
  const hasError = computed(() => status.value === "error");

  // Set up progress listener
  let unlistenProgress: (() => void) | null = null;

  async function setupProgressListener() {
    if (unlistenProgress) return;

    unlistenProgress = await listen<UploadProgress>(
      "upload-progress",
      (event) => {
        progress.value = event.payload;
      }
    );
  }

  function setFile(path: string) {
    filePath.value = path;

    // Extract file name from path
    const parts = path.split(/[/\\]/);
    fileName.value = parts[parts.length - 1];

    // Detect file type from extension
    const ext = fileName.value.split(".").pop()?.toLowerCase();
    if (ext) {
      if (["doc", "docx", "odt", "rtf", "txt"].includes(ext)) {
        fileType.value = "Google Docs";
      } else if (["xls", "xlsx", "ods", "csv", "tsv"].includes(ext)) {
        fileType.value = "Google Sheets";
      } else if (["ppt", "pptx", "odp"].includes(ext)) {
        fileType.value = "Google Slides";
      }
    }

    // Reset status
    status.value = "idle";
    result.value = null;
    error.value = null;
    progress.value = { bytes_uploaded: 0, total_bytes: 0, percentage: 0 };
  }

  async function upload(folderId?: string | null) {
    if (!filePath.value) {
      error.value = "No file selected";
      status.value = "error";
      return;
    }

    await setupProgressListener();

    status.value = "uploading";
    error.value = null;
    progress.value = { bytes_uploaded: 0, total_bytes: 0, percentage: 0 };

    try {
      const uploadResult = await invoke<UploadResult>("upload_file", {
        filePath: filePath.value,
        folderId: folderId || null,
      });

      result.value = uploadResult;
      status.value = "success";
    } catch (e) {
      error.value = String(e);
      status.value = "error";
    }
  }

  function reset() {
    filePath.value = null;
    fileName.value = null;
    fileType.value = null;
    fileSize.value = 0;
    status.value = "idle";
    progress.value = { bytes_uploaded: 0, total_bytes: 0, percentage: 0 };
    result.value = null;
    error.value = null;
  }

  function cleanup() {
    if (unlistenProgress) {
      unlistenProgress();
      unlistenProgress = null;
    }
  }

  return {
    filePath,
    fileName,
    fileType,
    fileSize,
    status,
    progress,
    result,
    error,
    isUploading,
    isComplete,
    hasError,
    setFile,
    upload,
    reset,
    cleanup,
  };
});
