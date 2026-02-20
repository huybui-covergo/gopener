<script setup lang="ts">
import { useSettingsStore, type RecentFile } from "../stores/settings";

const settingsStore = useSettingsStore();

async function openFile(file: RecentFile) {
  const { open } = await import("@tauri-apps/plugin-shell");
  await open(file.google_url);
}

function formatDate(timestamp: number) {
  const date = new Date(timestamp);
  const now = new Date();
  const diff = now.getTime() - date.getTime();

  // Less than 1 minute
  if (diff < 60000) {
    return "Just now";
  }

  // Less than 1 hour
  if (diff < 3600000) {
    const minutes = Math.floor(diff / 60000);
    return `${minutes}m ago`;
  }

  // Less than 24 hours
  if (diff < 86400000) {
    const hours = Math.floor(diff / 3600000);
    return `${hours}h ago`;
  }

  // Less than 7 days
  if (diff < 604800000) {
    const days = Math.floor(diff / 86400000);
    return `${days}d ago`;
  }

  // Otherwise show date
  return date.toLocaleDateString();
}

function getFileIcon(type: string) {
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
  <div class="recent-files">
    <div class="recent-header">
      <h2 class="recent-title">Recent Uploads</h2>
      <button
        v-if="settingsStore.recentFiles.length > 0"
        class="btn btn-ghost btn-sm"
        @click="settingsStore.clearRecentFiles()"
      >
        Clear All
      </button>
    </div>

    <div v-if="settingsStore.recentFiles.length === 0" class="empty-state">
      <div class="empty-state-icon">
        <svg
          width="48"
          height="48"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="1.5"
        >
          <path d="M12 8v4l3 3" />
          <circle cx="12" cy="12" r="10" />
        </svg>
      </div>
      <p class="empty-state-title">No recent uploads</p>
      <p class="empty-state-description">
        Files you upload will appear here for quick access
      </p>
    </div>

    <div v-else class="file-list">
      <div
        v-for="file in settingsStore.recentFiles"
        :key="file.id"
        class="file-item"
        @click="openFile(file)"
      >
        <div class="file-icon">{{ getFileIcon(file.file_type) }}</div>
        <div class="file-info">
          <p class="file-name">{{ file.name }}</p>
          <p class="file-meta">
            <span class="file-type">{{ file.file_type }}</span>
            <span class="separator">·</span>
            <span class="file-date">{{ formatDate(file.uploaded_at) }}</span>
          </p>
        </div>
        <div class="file-action">
          <svg
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <path
              d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"
            />
            <path d="M15 3h6v6" />
            <path d="M10 14L21 3" />
          </svg>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.recent-files {
  max-width: 600px;
  margin: 0 auto;
}

.recent-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
}

.recent-title {
  font-size: 16px;
  font-weight: 600;
  margin: 0;
}

.file-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all 0.15s ease;
}

.file-item:hover {
  background: var(--bg-hover);
  border-color: var(--border-color-strong);
}

.file-icon {
  font-size: 24px;
  flex-shrink: 0;
}

.file-info {
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

.file-meta {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 11px;
  color: var(--text-secondary);
  margin: 4px 0 0;
}

.separator {
  color: var(--text-tertiary);
}

.file-action {
  color: var(--text-tertiary);
  opacity: 0;
  transition: opacity 0.15s ease;
}

.file-item:hover .file-action {
  opacity: 1;
}
</style>
