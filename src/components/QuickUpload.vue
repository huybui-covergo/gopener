<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useUploadStore } from "../stores/upload";
import { useSettingsStore } from "../stores/settings";

const emit = defineEmits<{
  (e: "close"): void;
  (e: "browse-folder"): void;
}>();

const uploadStore = useUploadStore();
const settingsStore = useSettingsStore();

const destinationLabel = computed(() => {
  return settingsStore.defaultFolderName || "My Drive";
});

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

    // Auto-close if enabled
    if (settingsStore.autoCloseAfterUpload) {
      setTimeout(() => {
        emit("close");
        uploadStore.reset();
      }, 1500);
    }
  }
}

function handleCancel() {
  uploadStore.reset();
  emit("close");
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

onMounted(() => {
  // Auto-focus for keyboard navigation
});
</script>

<template>
  <div class="quick-upload">
    <div class="quick-upload-header">
      <h2 class="quick-upload-title">Quick Upload</h2>
      <button class="btn btn-ghost btn-sm" @click="handleCancel">
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

    <div class="quick-upload-content">
      <!-- File Info -->
      <div class="file-info">
        <div class="file-icon">{{ getFileIcon(uploadStore.fileType) }}</div>
        <div class="file-details">
          <p class="file-name">{{ uploadStore.fileName }}</p>
          <p class="file-type">Will convert to {{ uploadStore.fileType }}</p>
        </div>
      </div>

      <!-- Destination -->
      <div class="destination">
        <label class="form-label">Upload to</label>
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
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
            <path d="M22 4L12 14.01l-3-3" />
          </svg>
        </div>
        <span class="success-text">Uploaded!</span>
        <a
          :href="uploadStore.result?.web_view_link"
          target="_blank"
          class="btn btn-primary btn-sm"
        >
          Open
        </a>
      </div>

      <!-- Error -->
      <div v-else-if="uploadStore.hasError" class="upload-error">
        <p class="error-text">{{ uploadStore.error }}</p>
      </div>
    </div>

    <!-- Actions -->
    <div
      v-if="!uploadStore.isUploading && !uploadStore.isComplete"
      class="quick-upload-actions"
    >
      <button class="btn btn-secondary" @click="handleCancel">Cancel</button>
      <button class="btn btn-primary" @click="handleUpload">
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
        Upload
      </button>
    </div>

    <div v-else-if="uploadStore.isComplete" class="quick-upload-actions">
      <button
        class="btn btn-secondary"
        @click="
          () => {
            uploadStore.reset();
            emit('close');
          }
        "
      >
        Done
      </button>
    </div>
  </div>
</template>

<style scoped>
.quick-upload {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  width: 400px;
  box-shadow: var(--shadow-lg);
}

.quick-upload-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px;
  border-bottom: 1px solid var(--border-color);
}

.quick-upload-title {
  font-size: 14px;
  font-weight: 600;
  margin: 0;
}

.quick-upload-content {
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.file-info {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background: var(--bg-secondary);
  border-radius: var(--radius-md);
}

.file-icon {
  font-size: 28px;
}

.file-details {
  flex: 1;
  min-width: 0;
}

.file-name {
  font-size: 13px;
  font-weight: 500;
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.file-type {
  font-size: 11px;
  color: var(--text-secondary);
  margin: 2px 0 0;
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
  padding: 8px 10px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-size: 12px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.destination-button:hover {
  background: var(--bg-hover);
}

.destination-button .chevron {
  margin-left: auto;
  color: var(--text-tertiary);
}

.upload-progress {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.progress-text {
  font-size: 11px;
  color: var(--text-secondary);
  text-align: center;
}

.upload-success {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  background: rgba(34, 197, 94, 0.1);
  border-radius: var(--radius-md);
}

.success-icon {
  color: var(--success-color);
}

.success-text {
  font-size: 13px;
  font-weight: 500;
  color: var(--success-color);
  flex: 1;
}

.upload-error {
  padding: 12px;
  background: rgba(239, 68, 68, 0.1);
  border-radius: var(--radius-md);
}

.error-text {
  font-size: 12px;
  color: var(--error-color);
  margin: 0;
}

.quick-upload-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 16px;
  border-top: 1px solid var(--border-color);
}
</style>
