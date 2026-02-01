<script setup lang="ts">
import { ref, computed } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { useUploadStore } from "../stores/upload";
import { useSettingsStore } from "../stores/settings";

const emit = defineEmits<{
  (e: "browse-folder"): void;
}>();

const uploadStore = useUploadStore();
const settingsStore = useSettingsStore();

const isDragging = ref(false);

const supportedExtensions = [
  "doc",
  "docx",
  "odt",
  "rtf",
  "txt",
  "xls",
  "xlsx",
  "ods",
  "csv",
  "tsv",
  "ppt",
  "pptx",
  "odp",
];

const destinationLabel = computed(() => {
  return settingsStore.defaultFolderName || "My Drive";
});

async function handleFilePicker() {
  const selected = await open({
    multiple: false,
    filters: [
      {
        name: "Office Documents",
        extensions: supportedExtensions,
      },
    ],
  });

  if (selected && typeof selected === "string") {
    uploadStore.setFile(selected);
  }
}

function handleDragOver(e: DragEvent) {
  e.preventDefault();
  isDragging.value = true;
}

function handleDragLeave() {
  isDragging.value = false;
}

function handleDrop(e: DragEvent) {
  e.preventDefault();
  isDragging.value = false;

  const files = e.dataTransfer?.files;
  if (files && files.length > 0) {
    const file = files[0];
    // For Tauri, we need the file path which isn't available from drag events
    // We'll show a message to use the file picker instead
    console.log("Dropped file:", file.name);
    // In a real implementation, you'd use Tauri's file drop handling
  }
}

async function handleUpload() {
  await uploadStore.upload(settingsStore.defaultFolderId);

  if (uploadStore.isComplete && uploadStore.result) {
    // Add to recent files
    await settingsStore.addRecentFile({
      id: uploadStore.result.file_id,
      name: uploadStore.result.name,
      path: uploadStore.filePath || "",
      google_url: uploadStore.result.web_view_link,
      file_type: uploadStore.result.file_type,
      uploaded_at: Date.now(),
    });

    // Open in browser if enabled
    if (settingsStore.autoOpenAfterUpload) {
      const { open } = await import("@tauri-apps/plugin-shell");
      await open(uploadStore.result.web_view_link);
    }
  }
}

function handleReset() {
  uploadStore.reset();
}

function getFileIcon(type: string | null) {
  switch (type) {
    case "Google Docs":
      return "📄";
    case "Google Sheets":
      return "📊";
    case "Google Slides":
      return "📽️";
    default:
      return "📁";
  }
}
</script>

<template>
  <div class="file-uploader">
    <!-- Drop Zone -->
    <div
      v-if="!uploadStore.filePath"
      class="drop-zone"
      :class="{ dragging: isDragging }"
      @dragover="handleDragOver"
      @dragleave="handleDragLeave"
      @drop="handleDrop"
      @click="handleFilePicker"
    >
      <div class="drop-zone-content">
        <div class="drop-icon">
          <svg
            width="48"
            height="48"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1.5"
          >
            <path
              d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4M17 8l-5-5-5 5M12 3v12"
            />
          </svg>
        </div>
        <p class="drop-text">
          Drop an Office file here or <span class="link">browse</span>
        </p>
        <p class="drop-hint">
          Supports .doc, .docx, .xls, .xlsx, .ppt, .pptx, and more
        </p>
      </div>
    </div>

    <!-- File Selected -->
    <div v-else class="file-selected">
      <div class="file-info card">
        <div class="file-icon">{{ getFileIcon(uploadStore.fileType) }}</div>
        <div class="file-details">
          <p class="file-name">{{ uploadStore.fileName }}</p>
          <p class="file-type">Will convert to {{ uploadStore.fileType }}</p>
        </div>
        <button
          v-if="!uploadStore.isUploading && !uploadStore.isComplete"
          class="btn btn-ghost btn-sm"
          @click="handleReset"
        >
          <svg
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <path d="M18 6L6 18M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Destination -->
      <div class="destination">
        <label class="form-label">Destination</label>
        <button class="destination-button" @click="emit('browse-folder')">
          <svg
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <path
              d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"
            />
          </svg>
          {{ destinationLabel }}
          <svg
            width="12"
            height="12"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            class="chevron"
          >
            <path d="M9 18l6-6-6-6" />
          </svg>
        </button>
      </div>

      <!-- Progress -->
      <div v-if="uploadStore.isUploading" class="upload-progress">
        <div class="progress-bar">
          <div
            class="progress-bar-fill"
            :style="{ width: `${uploadStore.progress.percentage}%` }"
          ></div>
        </div>
        <p class="progress-text">
          Uploading... {{ Math.round(uploadStore.progress.percentage) }}%
        </p>
      </div>

      <!-- Success -->
      <div v-else-if="uploadStore.isComplete" class="upload-success">
        <div class="success-icon">
          <svg
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
            <path d="M22 4L12 14.01l-3-3" />
          </svg>
        </div>
        <p class="success-text">Uploaded successfully!</p>
        <div class="success-actions">
          <a
            :href="uploadStore.result?.web_view_link"
            target="_blank"
            class="btn btn-primary"
          >
            Open in {{ uploadStore.result?.file_type }}
          </a>
          <button class="btn btn-secondary" @click="handleReset">
            Upload Another
          </button>
        </div>
      </div>

      <!-- Error -->
      <div v-else-if="uploadStore.hasError" class="upload-error">
        <p class="error-text">{{ uploadStore.error }}</p>
        <button class="btn btn-secondary" @click="handleReset">
          Try Again
        </button>
      </div>

      <!-- Upload Button -->
      <div
        v-else
        class="upload-actions"
      >
        <button class="btn btn-primary btn-lg" @click="handleUpload">
          <svg
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <path
              d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4M17 8l-5-5-5 5M12 3v12"
            />
          </svg>
          Upload to Google Drive
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.file-uploader {
  max-width: 500px;
  margin: 0 auto;
}

.drop-zone {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 300px;
  border: 2px dashed var(--border-color);
  border-radius: var(--radius-lg);
  cursor: pointer;
  transition: all 0.2s ease;
}

.drop-zone:hover,
.drop-zone.dragging {
  border-color: var(--accent-color);
  background: var(--bg-hover);
}

.drop-zone-content {
  text-align: center;
  padding: 32px;
}

.drop-icon {
  color: var(--text-tertiary);
  margin-bottom: 16px;
}

.drop-text {
  font-size: 14px;
  color: var(--text-primary);
  margin-bottom: 8px;
}

.drop-text .link {
  color: var(--accent-color);
  text-decoration: underline;
}

.drop-hint {
  font-size: 12px;
  color: var(--text-secondary);
}

.file-selected {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.file-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.file-icon {
  font-size: 32px;
}

.file-details {
  flex: 1;
}

.file-name {
  font-size: 14px;
  font-weight: 500;
  margin: 0;
  word-break: break-all;
}

.file-type {
  font-size: 12px;
  color: var(--text-secondary);
  margin: 4px 0 0;
}

.destination {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.destination-button {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.destination-button:hover {
  background: var(--bg-hover);
  border-color: var(--border-color-strong);
}

.destination-button .chevron {
  margin-left: auto;
  color: var(--text-tertiary);
}

.upload-progress {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.progress-text {
  font-size: 12px;
  color: var(--text-secondary);
  text-align: center;
}

.upload-success {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 24px;
  text-align: center;
}

.success-icon {
  color: var(--success-color);
}

.success-text {
  font-size: 14px;
  font-weight: 500;
  color: var(--success-color);
}

.success-actions {
  display: flex;
  gap: 8px;
  margin-top: 8px;
}

.upload-error {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 24px;
}

.error-text {
  font-size: 13px;
  color: var(--error-color);
  text-align: center;
}

.upload-actions {
  display: flex;
  justify-content: center;
  margin-top: 8px;
}
</style>
