<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { useAuthStore } from "./stores/auth";
import { useSettingsStore } from "./stores/settings";
import { useUploadStore } from "./stores/upload";
import FileUploader from "./components/FileUploader.vue";
import QuickUpload from "./components/QuickUpload.vue";
import FolderBrowser from "./components/FolderBrowser.vue";
import RecentFiles from "./components/RecentFiles.vue";
import Settings from "./components/Settings.vue";
import AuthButton from "./components/AuthButton.vue";

const authStore = useAuthStore();
const settingsStore = useSettingsStore();
const uploadStore = useUploadStore();

type View = "upload" | "quick" | "settings" | "recent";
const currentView = ref<View>("upload");

const showFolderBrowser = ref(false);

let unlistenFileOpened: (() => void) | undefined;

// Check if app was opened with a file argument
onMounted(async () => {
  await authStore.checkAuth();
  await settingsStore.loadSettings();

  // Listen for file opened via CLI args
  const { listen } = await import("@tauri-apps/api/event");
  unlistenFileOpened = await listen<string>("file-opened", (event) => {
    uploadStore.setFile(event.payload);
    currentView.value = "quick";
  });
});

onUnmounted(() => {
  unlistenFileOpened?.();
});

const isAuthenticated = computed(() => authStore.isAuthenticated);
</script>

<template>
  <div class="app" :class="{ 'theme-dark': settingsStore.theme === 'dark' }">
    <header class="app-header">
      <div class="header-left">
        <h1 class="app-title">Gopener</h1>
        <nav class="nav-tabs">
          <button
            class="nav-tab"
            :class="{ active: currentView === 'upload' }"
            @click="currentView = 'upload'"
          >
            Upload
          </button>
          <button
            class="nav-tab"
            :class="{ active: currentView === 'recent' }"
            @click="currentView = 'recent'"
          >
            Recent
          </button>
          <button
            class="nav-tab"
            :class="{ active: currentView === 'settings' }"
            @click="currentView = 'settings'"
          >
            Settings
          </button>
        </nav>
      </div>
      <div class="header-right">
        <AuthButton />
      </div>
    </header>

    <main class="app-main">
      <template v-if="!isAuthenticated">
        <div class="auth-prompt">
          <div class="auth-icon">
            <svg
              width="64"
              height="64"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="1.5"
            >
              <path
                d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"
              />
            </svg>
          </div>
          <h2>Welcome to Gopener</h2>
          <p>Sign in with Google to upload Office files to Google Drive</p>
        </div>
      </template>

      <template v-else>
        <FileUploader
          v-if="currentView === 'upload'"
          @browse-folder="showFolderBrowser = true"
        />
        <QuickUpload
          v-else-if="currentView === 'quick'"
          @close="currentView = 'upload'"
          @browse-folder="showFolderBrowser = true"
        />
        <RecentFiles v-else-if="currentView === 'recent'" />
        <Settings v-else-if="currentView === 'settings'" />
      </template>
    </main>

    <!-- Folder Browser Modal -->
    <FolderBrowser
      v-if="showFolderBrowser"
      @close="showFolderBrowser = false"
      @select="
        (folder) => {
          settingsStore.setDefaultFolder(folder.id, folder.name);
          showFolderBrowser = false;
        }
      "
    />
  </div>
</template>

<style scoped>
.app {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: var(--bg-primary);
  color: var(--text-primary);
}

.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  height: 48px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  -webkit-app-region: drag;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 24px;
}

.header-right {
  -webkit-app-region: no-drag;
}

.app-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.nav-tabs {
  display: flex;
  gap: 4px;
  -webkit-app-region: no-drag;
}

.nav-tab {
  padding: 6px 12px;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
  background: transparent;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.nav-tab:hover {
  color: var(--text-primary);
  background: var(--bg-hover);
}

.nav-tab.active {
  color: var(--text-primary);
  background: var(--bg-active);
}

.app-main {
  flex: 1;
  overflow: auto;
  padding: 24px;
}

.auth-prompt {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  text-align: center;
  gap: 16px;
}

.auth-icon {
  color: var(--text-tertiary);
  margin-bottom: 8px;
}

.auth-prompt h2 {
  font-size: 20px;
  font-weight: 600;
  margin: 0;
}

.auth-prompt p {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
}
</style>
