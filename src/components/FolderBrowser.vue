<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface DriveFolder {
  id: string;
  name: string;
  mime_type: string;
}

const emit = defineEmits<{
  (e: "close"): void;
  (e: "select", folder: { id: string; name: string }): void;
}>();

const folders = ref<DriveFolder[]>([]);
const isLoading = ref(false);
const error = ref<string | null>(null);
const currentPath = ref<{ id: string | null; name: string }[]>([
  { id: null, name: "My Drive" },
]);
const newFolderName = ref("");
const showNewFolder = ref(false);
const isCreating = ref(false);

const currentFolderId = () => {
  const last = currentPath.value[currentPath.value.length - 1];
  return last.id;
};

async function loadFolders(parentId: string | null = null) {
  isLoading.value = true;
  error.value = null;

  try {
    folders.value = await invoke<DriveFolder[]>("list_folders", {
      parentId,
    });
  } catch (e) {
    error.value = String(e);
  } finally {
    isLoading.value = false;
  }
}

async function navigateToFolder(folder: DriveFolder) {
  currentPath.value.push({ id: folder.id, name: folder.name });
  await loadFolders(folder.id);
}

async function navigateToPath(index: number) {
  currentPath.value = currentPath.value.slice(0, index + 1);
  await loadFolders(currentFolderId());
}

async function createFolder() {
  if (!newFolderName.value.trim()) return;

  isCreating.value = true;
  try {
    const folder = await invoke<DriveFolder>("create_folder", {
      name: newFolderName.value.trim(),
      parentId: currentFolderId(),
    });

    // Add to list and select it
    folders.value.unshift(folder);
    newFolderName.value = "";
    showNewFolder.value = false;
  } catch (e) {
    error.value = String(e);
  } finally {
    isCreating.value = false;
  }
}

function selectCurrentFolder() {
  const current = currentPath.value[currentPath.value.length - 1];
  emit("select", { id: current.id || "root", name: current.name });
}

onMounted(() => {
  loadFolders();
});
</script>

<template>
  <div class="modal-overlay" @click.self="emit('close')">
    <div class="modal folder-browser">
      <div class="modal-header">
        <h3 class="modal-title">Select Destination Folder</h3>
        <button class="btn btn-ghost btn-sm" @click="emit('close')">
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

      <div class="modal-body">
        <!-- Breadcrumb -->
        <div class="breadcrumb">
          <button
            v-for="(item, index) in currentPath"
            :key="index"
            class="breadcrumb-item"
            @click="navigateToPath(index)"
          >
            {{ item.name }}
            <svg
              v-if="index < currentPath.length - 1"
              width="12"
              height="12"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <path d="M9 18l6-6-6-6" />
            </svg>
          </button>
        </div>

        <!-- New Folder Input -->
        <div v-if="showNewFolder" class="new-folder-input">
          <input
            v-model="newFolderName"
            type="text"
            class="input"
            placeholder="New folder name"
            @keyup.enter="createFolder"
            @keyup.escape="showNewFolder = false"
          />
          <button
            class="btn btn-primary btn-sm"
            @click="createFolder"
            :disabled="!newFolderName.trim() || isCreating"
          >
            Create
          </button>
          <button class="btn btn-ghost btn-sm" @click="showNewFolder = false">
            Cancel
          </button>
        </div>

        <!-- Loading -->
        <div v-if="isLoading" class="loading">
          <p>Loading folders...</p>
        </div>

        <!-- Error -->
        <div v-else-if="error" class="error">
          <p class="error-text">{{ error }}</p>
          <button class="btn btn-secondary btn-sm" @click="loadFolders(currentFolderId())">
            Retry
          </button>
        </div>

        <!-- Folder List -->
        <div v-else class="folder-list">
          <div
            v-for="folder in folders"
            :key="folder.id"
            class="folder-item"
            @click="navigateToFolder(folder)"
            @dblclick="emit('select', { id: folder.id, name: folder.name })"
          >
            <svg
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <path
                d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"
              />
            </svg>
            <span class="folder-name">{{ folder.name }}</span>
            <svg
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              class="chevron"
            >
              <path d="M9 18l6-6-6-6" />
            </svg>
          </div>

          <div v-if="folders.length === 0" class="empty-state">
            <p class="empty-state-description">No folders here</p>
          </div>
        </div>
      </div>

      <div class="modal-footer">
        <button
          v-if="!showNewFolder"
          class="btn btn-secondary"
          @click="showNewFolder = true"
        >
          <svg
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <path d="M12 5v14M5 12h14" />
          </svg>
          New Folder
        </button>
        <div class="spacer"></div>
        <button class="btn btn-secondary" @click="emit('close')">Cancel</button>
        <button class="btn btn-primary" @click="selectCurrentFolder">
          Select "{{ currentPath[currentPath.length - 1].name }}"
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.folder-browser {
  width: 500px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
}

.modal-body {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.breadcrumb {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 4px;
  padding: 8px 0;
  border-bottom: 1px solid var(--border-color);
}

.breadcrumb-item {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px;
  font-size: 12px;
  color: var(--text-secondary);
  background: transparent;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all 0.15s ease;
}

.breadcrumb-item:hover {
  color: var(--text-primary);
  background: var(--bg-hover);
}

.breadcrumb-item:last-child {
  color: var(--text-primary);
  font-weight: 500;
}

.new-folder-input {
  display: flex;
  gap: 8px;
  padding: 8px;
  background: var(--bg-secondary);
  border-radius: var(--radius-md);
}

.new-folder-input .input {
  flex: 1;
  font-size: 13px;
}

.loading,
.error {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 32px;
  color: var(--text-secondary);
}

.folder-list {
  flex: 1;
  overflow-y: auto;
}

.folder-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: background 0.15s ease;
}

.folder-item:hover {
  background: var(--bg-hover);
}

.folder-item svg:first-child {
  color: var(--text-secondary);
  flex-shrink: 0;
}

.folder-name {
  flex: 1;
  font-size: 13px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.folder-item .chevron {
  color: var(--text-tertiary);
  opacity: 0;
  transition: opacity 0.15s ease;
}

.folder-item:hover .chevron {
  opacity: 1;
}

.modal-footer {
  display: flex;
  align-items: center;
  gap: 8px;
}

.spacer {
  flex: 1;
}
</style>
